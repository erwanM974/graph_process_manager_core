/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::collections::HashSet;


use crate::process::config::AbstractProcessConfiguration;
use crate::process::logger::AbstractProcessLogger;
use crate::queue::delegate::ProcessQueueDelegate;
use crate::queue::memorized_node::MemorizedNode;
use crate::queue::priorities::GenericProcessPriorities;
use crate::queue::queued_step::EnqueuedStep;
use crate::queue::strategy::QueueSearchStrategy;

use crate::process::persistent_state::AbstractProcessMutablePersistentState;
use crate::process::handler::AbstractAlgorithmOperationHandler;

use super::filter::GenericFiltersManager;
use super::identifier::UniqueIdentifierGenerator;
use super::logger::*;
use super::node_memoizer::NodeMemoizer;


/** 
 * Entity responsible of the execution of the overall process.
 * **/
pub struct GenericProcessManager<Conf : AbstractProcessConfiguration> {
    context_and_param : Conf::ContextAndParameterization,
    // ***
    delegate : ProcessQueueDelegate<Conf::DomainSpecificStep,Conf::DomainSpecificNode,Conf::Priorities>,
    // ***
    global_state : Conf::MutablePersistentState,
    // ***
    filters_manager : GenericFiltersManager<Conf>,
    // ***
    loggers : Vec<Box< dyn AbstractProcessLogger<Conf>>>,
    // ***
    node_memoizer : NodeMemoizer<Conf>,
    // ***
    identifier_generator : UniqueIdentifierGenerator,
    // ***
    node_has_processed_child_tracker : HashSet<u32>
}



impl<Conf : 'static + AbstractProcessConfiguration> GenericProcessManager<Conf> {

    pub fn new(
        context_and_param : Conf::ContextAndParameterization,
        strategy: QueueSearchStrategy,
        priorities: GenericProcessPriorities<Conf::Priorities>,
        filters_manager : GenericFiltersManager<Conf>,
        loggers : Vec<Box< dyn AbstractProcessLogger<Conf>>>,
        is_memoized : bool
    ) -> GenericProcessManager<Conf> {
        let initial_global_state = Conf::MutablePersistentState::get_initial_state(&context_and_param);
        GenericProcessManager{
            context_and_param,
            delegate : ProcessQueueDelegate::new(strategy, priorities),
            global_state : initial_global_state,
            filters_manager,
            loggers,
            node_memoizer : NodeMemoizer::new(is_memoized),
            identifier_generator : UniqueIdentifierGenerator::new(),
            node_has_processed_child_tracker : HashSet::new()
        }
    }

    pub fn get_logger(&self, logger_id : usize) -> Option<&dyn AbstractProcessLogger<Conf>> {
        self.loggers.get(logger_id).map(|x| &**x)
    }

    pub fn start_process(
        &mut self,
        initial_node : Conf::DomainSpecificNode
    ) {

        loggers_initialize(
            self.loggers.iter_mut(),
            &self.context_and_param,
            self.delegate.get_strategy(),
            self.delegate.get_priorities(),
            &self.filters_manager,
            &self.global_state,
            self.node_memoizer.is_memoized()
        );
        
        let warrants_termination = {
            let new_node_id = self.identifier_generator.get_next();
            self.process_new_node_and_check_termination(
                initial_node,
                new_node_id
            )
        };

        if !warrants_termination {

            'process_step_loop : while let Some(
                (step_to_process,mut opt_parent_node)
            ) = self.delegate.extract_from_queue() {
                
                {
                    // this is isolated to avoid borrow checker problems

                    let parent_node =
                    opt_parent_node.as_mut().unwrap_or_else(|| self.delegate.get_mut_memorized_node(step_to_process.parent_node_id));
                    
                    // we will process the step that may be fired from the parent node
                    // in any case, we update the parent node's remainign to process childrens
                    parent_node.remaining_child_steps_ids_to_process.remove(&step_to_process.id_as_potential_step_from_parent);
                }

                // we need an immutable reference to the parent node
                // but it may be under self.delegate
                // so then when calling "self.process_step_and_check_termination(step_to_process,parent_node)"
                // we run into borrow checker problem
                // for now the solution is to clone the node even though not ideal
                let parent_node_clone = match opt_parent_node {
                    None => {
                        self.delegate.get_memorized_node(step_to_process.parent_node_id).clone()
                    },
                    Some(x) => {
                        x
                    }
                };

                let warrants_termination_inner = self.process_step_and_check_termination(
                    step_to_process,
                    &parent_node_clone
                );
                if warrants_termination_inner {
                    break 'process_step_loop;
                }
            }

        }

        loggers_terminate_process(
            self.loggers.iter_mut(),
            &self.context_and_param,
            &self.global_state
        );
    }

    

    fn process_step_and_check_termination(
        &mut self,
        step_to_process : EnqueuedStep<Conf::DomainSpecificStep>,
        parent_node : &MemorizedNode<Conf::DomainSpecificNode>
    ) -> bool {
        // apply the step filters
        let warrants_termination = match self.filters_manager.apply_step_filters(
            &self.context_and_param,
            &self.global_state,
            &parent_node.domain_specific_node,
            &step_to_process.domain_specific_step
        ) {
            Some(filtration_result) => {
                // here, a filter was activated
                // this means that we won't explore further the successors from this specific step
                // ***
                // below we notify the loggers
                let filtration_result_id = self.identifier_generator.get_next();
                loggers_filtered(
                    self.loggers.iter_mut(), 
                    &self.context_and_param,
                    step_to_process.parent_node_id,
                    filtration_result_id, 
                    &filtration_result
                );
                // and we update the global state
                self.global_state.update_on_filtered(
                    &self.context_and_param,
                    &parent_node.domain_specific_node,
                    &filtration_result
                );
                // the filtration may warrant process termination
                self.global_state.warrants_termination_of_the_process(&self.context_and_param)
            },
            None => {
                // here there are no filter that prevent the firing of the step
                // ***
                // because we can process it, this means that the parent node of the step (from which the step is fired)
                // is guaranteed to have at least one child
                // thus we update the tracker
                self.node_has_processed_child_tracker.insert(step_to_process.id_as_potential_step_from_parent);
                // ***
                // processing the step yields a successor node
                // thus we process it to get the successor node
                let successor_node = Conf::AlgorithmOperationHandler::process_new_step(
                    &self.context_and_param,
                    &parent_node.domain_specific_node,
                    &step_to_process.domain_specific_step
                );
                // now, if the memoization option is active,
                // we check if this node has already been reached previously
                // and return the id of the successor node
                let (successor_node_id,check_termination) = match self.node_memoizer.check_memo(&successor_node) {
                    Some(memoized_node_id) => {
                        // here the sucessor node is already known and memoized, so we return its unique id
                        // also because the global state is not updated, termination is not warranted
                        (memoized_node_id,false)
                    },
                    None => {
                        // here the successor node is entirely new
                        // so we create a new unique identifier
                        let new_node_id = self.identifier_generator.get_next();
                        // here the fact that we have a new node
                        // requires us to check termination
                        (new_node_id,true)
                    },
                };
                // now that we have the "successor_node_id", we can log the new step
                loggers_new_step(
                    self.loggers.iter_mut(),
                    &self.context_and_param,
                    step_to_process.parent_node_id,
                    &step_to_process.domain_specific_step,
                    successor_node_id,
                    &successor_node
                );
                // ***
                let warrants_termination = if check_termination {
                    // here we process the new node further
                    // and incidentally check termination
                    self.process_new_node_and_check_termination(
                        successor_node,
                        successor_node_id
                    )
                } else {
                    false
                };
                // and we propagate "warrants_termination"
                warrants_termination
            }
        };
        // ***
        if parent_node.remaining_child_steps_ids_to_process.is_empty() {
            let parent_had_at_least_one_processed_child = self.node_has_processed_child_tracker.remove(
                &step_to_process.id_as_potential_step_from_parent
            );
            if !parent_had_at_least_one_processed_child {
                // for the HCS queue to know the node id'ed by parent_id is terminal
                self.delegate.queue_set_last_reached_has_no_child();
            }
            loggers_notify_last_child_step_of_node_processed(
                self.loggers.iter_mut(),
                &self.context_and_param,
                step_to_process.parent_node_id
            )
        }
        // and we propagate "warrants_termination"
        warrants_termination
    }



    fn process_new_node_and_check_termination(
        &mut self,
        new_node : Conf::DomainSpecificNode,
        new_node_id : u32
    ) -> bool {
        // we notify the memoizer of the new node (actually memoizes only if the memoizer is active)
        self.node_memoizer.memoize_new_node(&new_node,new_node_id);
        // we notify the loggers of the new node
        loggers_new_node(
            self.loggers.iter_mut(),
            &self.context_and_param, 
            new_node_id, 
            &new_node
        );
        // we update the global state
        self.global_state.update_on_node_reached(
            &self.context_and_param,
            &new_node
        );
        // updating the global state may warrant termination
        if self.global_state.warrants_termination_of_the_process(&self.context_and_param) {
            return true;
        }
        // ***
        // here it does not warrant termination
        // so we process the new node further
        // ***
        // we apply the node pre filters
        let (has_no_children,warrants_termination) = match self.filters_manager.apply_node_pre_filters(
            &self.context_and_param,
            &self.global_state,
            &new_node
        ) {
            Some(filtration_result) => {
                // here, a filter was activated
                // this means that we won't explore further the successors from this specific node
                // ***
                // below we notify the loggers of the filtration
                let filtration_result_id = self.identifier_generator.get_next();
                loggers_filtered(
                    self.loggers.iter_mut(), 
                    &self.context_and_param,
                    new_node_id,
                    filtration_result_id, 
                    &filtration_result
                );
                // and we update the global state
                self.global_state.update_on_filtered(
                    &self.context_and_param,
                    &new_node,
                    &filtration_result
                );
                // the filtration may warrant process termination
                let warrants_termination = self.global_state.warrants_termination_of_the_process(&self.context_and_param);
                // ***
                (true,warrants_termination)
            },
            None => {
                // here no node pre filters were activated
                // so we can collect the next steps that may be fired from that node
                let next_steps = Conf::AlgorithmOperationHandler::collect_next_steps(
                    &self.context_and_param,
                    &new_node
                );
                // we apply the node post filters
                match self.filters_manager.apply_node_post_filters(
                    &self.context_and_param,
                    &self.global_state,
                    &new_node,
                    &next_steps
                ) {
                    Some(filtration_result) => {
                        // here, a filter was activated
                        // this means that we won't explore further the successors from this specific node
                        // ***
                        // below we notify the loggers of the filtration
                        let filtration_result_id = self.identifier_generator.get_next();
                        loggers_filtered(
                            self.loggers.iter_mut(), 
                            &self.context_and_param,
                            new_node_id,
                            filtration_result_id, 
                            &filtration_result
                        );
                        // and we update the global state
                        self.global_state.update_on_filtered(
                            &self.context_and_param,
                            &new_node,
                            &filtration_result
                        );
                        // the filtration may warrant process termination
                        let warrants_termination = self.global_state.warrants_termination_of_the_process(&self.context_and_param);
                        // ***
                        (true,warrants_termination)
                    },
                    None => {
                        // here no node post filters were activated
                        // this means we can enqueue all these next steps
                        // if there are any
                        if next_steps.is_empty() {
                            (true,false)
                        } else {
                            let mut to_enqueue = vec![];
                            let mut max_id_of_child = 0;
                            for domain_specific_step in next_steps {
                                max_id_of_child += 1;
                                to_enqueue.push( 
                                    EnqueuedStep::new(
                                        new_node_id, 
                                        max_id_of_child, 
                                        domain_specific_step
                                    )
                                );
                            }
                            let remaining_ids_to_process : HashSet<u32> = HashSet::from_iter((1..(max_id_of_child+1)).collect::<Vec<u32>>().iter().cloned() );
                            let memorized_node = MemorizedNode::new(
                                new_node,
                                remaining_ids_to_process
                            );
                            self.delegate.enqueue_new_steps(
                                memorized_node,
                                new_node_id,
                                to_enqueue
                            );
                            (false,true)
                        }
                    }
                }
            }
        };
        if has_no_children {
            // the node does not have any children : it is a terminal node
            // notifies the queue
            self.delegate.queue_set_last_reached_has_no_child();
            // notifies the loggers
            loggers_notify_node_without_children(
                self.loggers.iter_mut(),
                &self.context_and_param,
                new_node_id
            );
        }
        // and we propagate "warrants_termination"
        warrants_termination
    }

}