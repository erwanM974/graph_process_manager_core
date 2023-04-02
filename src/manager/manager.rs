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

use crate::config::{AbstractConfiguration, AbstractNodeKind};
use crate::delegate::delegate::GenericProcessDelegate;
use crate::manager::handler::AbstractProcessHandler;
use crate::manager::verdict::AbstractGlobalVerdict;
use crate::node::GenericNode;
use crate::logger::AbstractProcessLogger;
use crate::manager::retval::GenericProcessReturnValue;


pub struct GenericProcessManager<Config : AbstractConfiguration> {
    process_context : Config::ProcessContext,
    process_parameterization : Config::ProcessParameterization,
    // ***
    delegate : GenericProcessDelegate<Config>,
    loggers : Vec<Config::Logger>,
    goal : Option<Config::GlobalVerdict>,
    // ***
    memoized : Option<HashMap<Config::NodeKind,u32>>,
    // ***
    has_filtered_nodes : bool,
    node_has_processed_child : HashSet<u32>
}



impl<Config : 'static + AbstractConfiguration> GenericProcessManager<Config> {

    pub fn new(process_context : Config::ProcessContext,
               process_parameterization : Config::ProcessParameterization,
               delegate : GenericProcessDelegate<Config>,
               loggers : Vec<Config::Logger>,
               goal : Option<Config::GlobalVerdict>,
               is_memoized : bool) -> GenericProcessManager<Config> {
        let memoized : Option<HashMap<Config::NodeKind,u32>>;
        if is_memoized {
            memoized = Some(hashmap!{});
        } else {
            memoized = None;
        }
        return GenericProcessManager{process_context,
            process_parameterization,
            delegate,
            loggers,
            goal,
            memoized,
            has_filtered_nodes:false,
            node_has_processed_child:hashset!{}};
    }

    pub fn get_memoized(&self) -> &Option<HashMap<Config::NodeKind,u32>> {
        return &self.memoized;
    }

    fn check_memo(memo : &HashMap<Config::NodeKind,u32>, to_look_up : &Config::NodeKind) -> Option<u32> {
        for (memoized_node, memoized_node_id) in memo {
            if to_look_up.is_included_for_memoization(memoized_node) {
                return Some(*memoized_node_id);
            }
        }
        return None;
    }

    pub fn start_process(&mut self,
                         init_node_kind : Config::NodeKind)
                -> GenericProcessReturnValue<Config> {

        let mut next_state_id : u32 = 1;
        let mut node_counter : u32 = 0;

        self.loggers_parameterization();
        self.loggers_initialize(next_state_id,&init_node_kind);

        let mut global_verdict = Config::GlobalVerdict::get_baseline_verdict();

        match &mut self.memoized {
            None => {},
            Some( memo ) => {
                memo.insert(init_node_kind.clone(), next_state_id);
            }
        }

        let pursue_analysis: bool;
        match self.enqueue_next_steps_from_current_node(next_state_id,init_node_kind,0) {
            None => {
                pursue_analysis = true;
            },
            Some(local_verdict) => {
                global_verdict = global_verdict.update_with_local_verdict(&local_verdict);
                pursue_analysis = ! global_verdict.is_goal_reached(&self.goal);
            }
        }
        next_state_id = next_state_id +1;
        node_counter = node_counter +1;


        if pursue_analysis {
            while let Some(step_to_process) = self.delegate.extract_from_queue() {
                let new_state_id = next_state_id;
                next_state_id = next_state_id + 1;
                // ***
                let mut parent_state = self.delegate.pick_memorized_state(step_to_process.parent_id);
                // ***
                match Config::ProcessHandler::process_new_step(&parent_state,&step_to_process,new_state_id,node_counter) {
                    Err(filter_elimination) => {
                        self.loggers_filtered(step_to_process.parent_id,
                                              new_state_id,
                                              &filter_elimination);
                    },
                    Ok(new_node_kind) => {
                        // ***
                        self.node_has_processed_child.insert(step_to_process.parent_id);

                        // check if this node has already been reached if graph is memoized
                        let node_already_known_in_memoized : Option<u32>;
                        match &self.memoized {
                            None => {
                                node_already_known_in_memoized = None;
                            },
                            Some( memo ) => {
                                node_already_known_in_memoized = Self::check_memo(memo, &new_node_kind);
                            }
                        }

                        // ***
                        match node_already_known_in_memoized {
                            None => {
                                let child_depth = parent_state.depth + 1;
                                node_counter = node_counter + 1;

                                self.loggers_new_node_added(new_state_id,&new_node_kind);
                                self.loggers_new_transition_added(step_to_process.parent_id,
                                                                  new_state_id,
                                                                  &step_to_process.kind);

                                match self.enqueue_next_steps_from_current_node(new_state_id,
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
                                self.loggers_new_transition_added(step_to_process.parent_id,
                                                                  memorized_node_id,
                                                                  &step_to_process.kind);
                            }
                        }

                    }
                }
                // ***
                parent_state.remaining_ids_to_process.remove(&step_to_process.id_as_child);
                if parent_state.remaining_ids_to_process.len() > 0 {
                    self.delegate.remember_state(step_to_process.parent_id,parent_state);
                } else {
                    let parent_had_at_least_one_processed_child = self.node_has_processed_child.remove(&step_to_process.parent_id);
                    if !parent_had_at_least_one_processed_child {
                        self.loggers_notify_terminal_node_reached(step_to_process.parent_id);
                    }
                    self.loggers_notify_last_child_of_node_processed(step_to_process.parent_id);
                }
            }
        }

        // ***
        global_verdict = global_verdict.update_knowing_nodes_were_filtered_out(self.has_filtered_nodes);

        self.loggers_terminate(&global_verdict);

        return GenericProcessReturnValue::new(node_counter,global_verdict);
    }


    fn enqueue_next_steps_from_current_node(&mut self,
                                       current_node_id : u32,
                                       current_node_kind : Config::NodeKind,
                                       depth : u32) -> Option<Config::LocalVerdict> {
        // ***
        let (max_id_of_child, new_steps) = Config::ProcessHandler::collect_next_steps(current_node_id,&current_node_kind);
        // ***
        if max_id_of_child > 0 {
            let remaining_ids_to_process : HashSet<u32> = HashSet::from_iter((1..(max_id_of_child+1)).collect::<Vec<u32>>().iter().cloned() );
            let generic_node = GenericNode::new(current_node_kind,remaining_ids_to_process,depth);
            self.delegate.remember_state( current_node_id, generic_node );
            self.delegate.enqueue_new_steps( current_node_id, new_steps );
            // ***
            return None;
        } else {
            // for the HCS queue to know the node id'ed by parent_id is terminal
            self.delegate.queue_set_last_reached_has_no_child();
            // notify loggers that a terminal node has been reached
            self.loggers_notify_terminal_node_reached(current_node_id);
            // ***
            let local_verdict = Config::ProcessHandler::get_local_verdict(&self.process_context,&current_node_kind);
            self.loggers_verdict(current_node_id,&local_verdict);
            return Some(local_verdict);
        }
    }

    fn loggers_notify_last_child_of_node_processed(&mut self, state_id : u32) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_notify_last_child_of_node_processed(&self.process_context,state_id);
        }
    }

    fn loggers_notify_terminal_node_reached(&mut self, state_id : u32) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_notify_terminal_node_reached(&self.process_context,state_id);
        }
    }

    fn loggers_new_node_added(&mut self,
                              new_node_id :u32,
                              new_node : &Config::NodeKind) {
        for logger in self.loggers.iter_mut() {
            logger.log_new_node(&self.process_context,
                                new_node_id,
                                new_node);
        }
    }

    fn loggers_new_transition_added(&mut self,
                                    origin_node_id :u32,
                                    target_node_id : u32,
                                    new_step : &Config::StepKind) {
        for logger in self.loggers.iter_mut() {
            logger.log_new_transition(&self.process_context,
                                      origin_node_id,
                                      target_node_id,
                                      new_step);
        }
    }

    fn loggers_verdict(&mut self,
                       parent_state_id : u32,
                       local_verdict : &Config::LocalVerdict) {
        for logger in self.loggers.iter_mut() {
            logger.log_verdict(&self.process_context,
                               parent_state_id,
                               local_verdict);
        }
    }

    fn loggers_initialize(&mut self, init_state_id : u32, init_node : &Config::NodeKind) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_initialize(&self.process_context,
                               init_state_id,
                               init_node);
        }
    }

    fn loggers_parameterization(&mut self) {
        let strategy = self.delegate.get_strategy();
        let filters = self.delegate.get_filters();
        let priorities = self.delegate.get_priorities();
        for logger in self.loggers.iter_mut() {
            (*logger).log_parameterization(strategy,
                                           filters,
                                           priorities,
                                           &self.process_parameterization);
        }
    }

    fn loggers_filtered(&mut self,
                        parent_state_id : u32,
                        new_state_id : u32,
                        elim_kind : &Config::FilterEliminationKind) {
        self.has_filtered_nodes = true;
        for logger in self.loggers.iter_mut() {
            logger.log_filtered(&self.process_context,
                                parent_state_id,
                                new_state_id,
                                elim_kind);
        }
    }

    fn loggers_terminate(&mut self, global_verdict : &Config::GlobalVerdict) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_terminate(global_verdict);
        }
    }

}