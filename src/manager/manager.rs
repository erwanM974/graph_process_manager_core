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

use std::collections::{HashMap, HashSet};

use crate::delegate::delegate::GenericProcessDelegate;
use crate::delegate::node::GenericNode;
use crate::handler::filter::AbstractFilter;
use crate::handler::handler::AbstractProcessHandler;
use crate::manager::config::{AbstractProcessConfiguration, AbstractNodeKind};
use crate::manager::logger::AbstractProcessLogger;
use crate::manager::verdict::AbstractGlobalVerdict;


pub struct GenericProcessManager<Conf : AbstractProcessConfiguration> {
    context : Conf::Context,
    param : Conf::Parameterization,
    // ***
    delegate : GenericProcessDelegate<Conf::StepKind,Conf::NodeKind,Conf::Priorities>,
    filters : Vec<Box<dyn AbstractFilter<Conf::FilterCriterion,Conf::FilterEliminationKind>>>,
    loggers : Vec<Box< dyn AbstractProcessLogger<Conf>>>,
    goal : Option<Conf::GlobalVerdict>,
    // ***
    memoized : Option<HashMap<Conf::NodeKind,u32>>,
    // ***
    has_filtered_nodes : bool,
    node_has_processed_child : HashSet<u32>
}



impl<Conf : 'static + AbstractProcessConfiguration> GenericProcessManager<Conf> {

    pub fn new(context : Conf::Context,
               param : Conf::Parameterization,
               delegate : GenericProcessDelegate<Conf::StepKind,Conf::NodeKind,Conf::Priorities>,
               filters : Vec<Box<dyn AbstractFilter<Conf::FilterCriterion,Conf::FilterEliminationKind>>>,
               loggers : Vec<Box< dyn AbstractProcessLogger<Conf>>>,
               goal : Option<Conf::GlobalVerdict>,
               is_memoized : bool) -> GenericProcessManager<Conf> {

        let memoized : Option<HashMap<Conf::NodeKind,u32>> = if is_memoized {
            Some(hashmap!{})
        } else {
            None
        };

        GenericProcessManager{context,
            param,
            delegate,
            filters,
            loggers,
            goal,
            memoized,
            has_filtered_nodes:false,
            node_has_processed_child:hashset!{}}
    }

    pub fn get_logger(&self, logger_id : usize) -> Option<&Box< dyn AbstractProcessLogger<Conf>>> {
        self.loggers.get(logger_id)
    }

    fn check_memo(memo : &HashMap<Conf::NodeKind,u32>, to_look_up : &Conf::NodeKind) -> Option<u32> {
        for (memoized_node, memoized_node_id) in memo {
            if to_look_up.is_included_for_memoization(memoized_node) {
                return Some(*memoized_node_id);
            }
        }
        None
    }

    pub fn start_process(&mut self,
                         init_node_kind : Conf::NodeKind)
                -> (u32,Conf::GlobalVerdict) {

        let mut next_node_id : u32 = 1;
        let mut node_counter : u32 = 0;

        self.loggers_initialize();
        self.loggers_parameterization();
        self.loggers_new_node(next_node_id,&init_node_kind);

        let mut global_verdict = Conf::GlobalVerdict::get_baseline_verdict();

        match &mut self.memoized {
            None => {},
            Some( memo ) => {
                memo.insert(init_node_kind.clone(), next_node_id);
            }
        }

        let pursue_analysis: bool;
        match self.enqueue_next_steps_from_current_node(next_node_id,init_node_kind,0) {
            None => {
                pursue_analysis = true;
            },
            Some(local_verdict) => {
                global_verdict = global_verdict.update_with_local_verdict(&local_verdict);
                pursue_analysis = ! global_verdict.is_goal_reached(&self.goal);
            }
        }
        next_node_id += 1;
        node_counter += 1;


        if pursue_analysis {
            while let Some(step_to_process) = self.delegate.extract_from_queue() {
                let new_node_id = next_node_id;
                next_node_id += 1;
                // ***
                let mut parent_node = self.delegate.pop_memorized_state(step_to_process.parent_id);
                // ***
                let criterion = Conf::ProcessHandler::get_criterion(&self.context,
                                                                      &self.param,
                                                                      &parent_node,
                                                                      &step_to_process,
                                                                      new_node_id,
                                                                      node_counter);
                let child_depth = parent_node.depth + 1;
                match self.apply_filters(child_depth,node_counter,&criterion) {
                    Some(filter_elimination) => {
                        self.loggers_filtered(step_to_process.parent_id,
                                              new_node_id,
                                              &filter_elimination);
                    },
                    None => {
                        let new_node_kind = Conf::ProcessHandler::process_new_step(&self.context,
                                                                                     &self.param,
                                                                                     &parent_node,
                                                                                     &step_to_process,
                                                                                     new_node_id,
                                                                                     node_counter);

                        self.node_has_processed_child.insert(step_to_process.parent_id);

                        // check if this node has already been reached if graph is memoized
                        let node_already_known_in_memoized : Option<u32> = match &self.memoized {
                            None => {
                                None
                            },
                            Some( memo ) => {
                                Self::check_memo(memo, &new_node_kind)
                            }
                        };

                        // ***
                        match node_already_known_in_memoized {
                            None => {
                                node_counter += 1;

                                if let Some(memo) = &mut self.memoized {
                                    memo.insert(new_node_kind.clone(), new_node_id);
                                }

                                self.loggers_new_node(new_node_id,&new_node_kind);
                                self.loggers_new_step(step_to_process.parent_id,
                                                                  new_node_id,
                                                                  &step_to_process.kind,
                                                                  &new_node_kind,
                                                      child_depth);

                                match self.enqueue_next_steps_from_current_node(new_node_id,
                                                                                new_node_kind,
                                                                                child_depth) {
                                    None => {},
                                    Some(local_verdict) => {
                                        global_verdict = global_verdict.update_with_local_verdict(&local_verdict);
                                        if global_verdict.is_goal_reached(&self.goal) {
                                            break;
                                        }
                                    }
                                }
                            },
                            Some( memorized_node_id) => {
                                self.loggers_new_step(step_to_process.parent_id,
                                                                  memorized_node_id,
                                                                  &step_to_process.kind,
                                                                  &new_node_kind,
                                                      child_depth);
                            }
                        }

                    }
                }
                // ***
                parent_node.remaining_ids_to_process.remove(&step_to_process.id_as_child);

                // ***
                if parent_node.remaining_ids_to_process.is_empty() {
                    let parent_had_at_least_one_processed_child = self.node_has_processed_child.remove(&step_to_process.parent_id);
                    if !parent_had_at_least_one_processed_child {
                        self.loggers_notify_terminal_node_reached(step_to_process.parent_id);
                    }
                    self.loggers_notify_last_child_of_node_processed(step_to_process.parent_id);
                } else {
                    self.delegate.remember_state(step_to_process.parent_id,parent_node);
                }

            }
        }

        // ***
        global_verdict = global_verdict.update_knowing_nodes_were_filtered_out(self.has_filtered_nodes);

        self.loggers_terminate(&global_verdict);

        // ***
        (node_counter,global_verdict)
    }

    fn apply_filters(&self,
                     depth : u32,
                     node_counter : u32,
                     criterion : &Conf::FilterCriterion) -> Option<Conf::FilterEliminationKind> {
        for filter in &self.filters {
            match filter.apply_filter(depth,node_counter,criterion) {
                None => {},
                Some( elim_kind) => {
                    return Some(elim_kind);
                }
            }
        }
        None
    }

    fn enqueue_next_steps_from_current_node(&mut self,
                                       current_node_id : u32,
                                       current_node_kind : Conf::NodeKind,
                                       depth : u32) -> Option<Conf::LocalVerdict> {
        let mut current_node_kind = current_node_kind;
        // ***
        let (max_id_of_child, new_steps) = Conf::ProcessHandler::collect_next_steps(&self.context,
                                                                                      &self.param,
                                                                                      current_node_id,
                                                                                      &current_node_kind);
        // ***
        if max_id_of_child > 0 {

            let local_verdict : Option<Conf::LocalVerdict>;
            let pursue_process : bool =
            match Conf::ProcessHandler::get_local_verdict_from_static_analysis(&self.context,
                                                                               &self.param,
                                                                               &mut current_node_kind) {
                None => {
                    local_verdict = None;
                    true
                },
                Some((static_verdict,proof_data)) => {
                    self.loggers_verdict(current_node_id,&static_verdict, Some(proof_data));
                    let pursue = Conf::ProcessHandler::pursue_process_after_static_verdict(&self.context,
                                                                                           &self.param,
                                                                                           &static_verdict);
                    local_verdict = Some(static_verdict);
                    pursue
                }
            };

            if pursue_process {
                let remaining_ids_to_process : HashSet<u32> = HashSet::from_iter((1..(max_id_of_child+1)).collect::<Vec<u32>>().iter().cloned() );
                let generic_node = GenericNode::new(current_node_kind,remaining_ids_to_process,depth);
                self.delegate.remember_state( current_node_id, generic_node );
                self.delegate.enqueue_new_steps( current_node_id, new_steps );
            } else {
                // for the HCS queue to know the node id'ed by parent_id is terminal
                self.delegate.queue_set_last_reached_has_no_child();
                // notify loggers that a terminal node has been reached
                self.loggers_notify_terminal_node_reached(current_node_id);
                //
            }

            local_verdict
        } else {
            // for the HCS queue to know the node id'ed by parent_id is terminal
            self.delegate.queue_set_last_reached_has_no_child();
            // notify loggers that a terminal node has been reached
            self.loggers_notify_terminal_node_reached(current_node_id);
            // ***
            let local_verdict = Conf::ProcessHandler::get_local_verdict_when_no_child(&self.context,
                                                                                        &self.param,
                                                                                        &current_node_kind);
            self.loggers_verdict(current_node_id, &local_verdict, None);
            // ***
            Some(local_verdict)
        }
    }

    fn loggers_notify_last_child_of_node_processed(&mut self, node_id : u32) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_notify_last_child_of_node_processed(&self.context,node_id);
        }
    }

    fn loggers_notify_terminal_node_reached(&mut self, node_id : u32) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_notify_terminal_node_reached(&self.context, node_id);
        }
    }

    fn loggers_new_node(&mut self,
                        new_node_id :u32,
                        new_node : &Conf::NodeKind) {
        for logger in self.loggers.iter_mut() {
            logger.log_new_node(&self.context,
                                &self.param,
                                new_node_id,
                                new_node);
        }
    }

    fn loggers_new_step(&mut self,
                        origin_node_id :u32,
                        target_node_id : u32,
                        new_step : &Conf::StepKind,
                        target_node : &Conf::NodeKind,
                        target_depth : u32) {
        for logger in self.loggers.iter_mut() {
            logger.log_new_step(&self.context,
                                &self.param,
                                origin_node_id,
                                target_node_id,
                                new_step,
                                target_node,
                                target_depth);
        }
    }

    fn loggers_verdict(&mut self,
                       parent_node_id : u32,
                       local_verdict : &Conf::LocalVerdict,
                       proof : Option<Conf::StaticLocalVerdictAnalysisProof>) {
        match proof {
            None => {
                for logger in self.loggers.iter_mut() {
                    logger.log_verdict_on_no_child(&self.context,
                                                   &self.param,
                                                   parent_node_id,
                                                   local_verdict);
                }
            },
            Some(data) => {
                for logger in self.loggers.iter_mut() {
                    logger.log_verdict_on_static_analysis(&self.context,
                                                          &self.param,
                                                          parent_node_id,
                                                          local_verdict,&data);
                }
            }
        }
    }

    fn loggers_parameterization(&mut self) {
        let strategy = self.delegate.get_strategy();
        let priorities = self.delegate.get_priorities();
        let use_memoization = self.memoized.is_some();
        for logger in self.loggers.iter_mut() {
            (*logger).log_parameterization(strategy,
                                           priorities,
                                           &self.filters,&self.goal,
                                           use_memoization,
                                           &self.param);
        }
    }

    fn loggers_initialize(&mut self) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_initialize();
        }
    }

    fn loggers_filtered(&mut self,
                        parent_node_id : u32,
                        new_node_id : u32,
                        elim_kind : &Conf::FilterEliminationKind) {
        self.has_filtered_nodes = true;
        for logger in self.loggers.iter_mut() {
            logger.log_filtered(&self.context,
                                parent_node_id,
                                new_node_id,
                                elim_kind);
        }
    }

    fn loggers_terminate(&mut self, global_verdict : &Conf::GlobalVerdict) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_terminate(global_verdict);
        }
    }

}