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

use std::collections::HashMap;
use maplit::hashmap;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::config::{AbstractConfiguration, AbstractStepKind};
use crate::delegate::filter::AbstractFilter;

use crate::delegate::priorities::{GenericProcessPriorities};
use crate::delegate::queue::factory::create_process_queue;
use crate::delegate::queue::generic::GenericProcessQueue;
use crate::delegate::queue::strategy::QueueSearchStrategy;
use crate::node::GenericNode;
use crate::step::GenericStep;


pub struct GenericProcessDelegate<Config : AbstractConfiguration> {
    strategy : QueueSearchStrategy,
    filters : Vec<Config::Filter>,
    priorities : GenericProcessPriorities<Config>,
    memorized_states : HashMap<u32,GenericNode<Config>>,
    process_queue : Box< dyn GenericProcessQueue<Config> >
}

impl<Config : 'static + AbstractConfiguration> GenericProcessDelegate<Config> {

    pub fn new(strategy : QueueSearchStrategy,
               filters : Vec<Config::Filter>,
               priorities : GenericProcessPriorities<Config>)
            -> GenericProcessDelegate<Config> {
        let process_queue = create_process_queue(&strategy);
        return GenericProcessDelegate{
            strategy,
            filters,
            priorities,
            memorized_states:hashmap!{},
            process_queue};
    }

    pub fn get_strategy(&self) -> &QueueSearchStrategy {
        return &self.strategy;
    }

    pub fn get_filters(&self) -> &Vec<Config::Filter> {
        return &self.filters;
    }

    pub fn get_priorities(&self) -> &GenericProcessPriorities<Config> {
        return &self.priorities;
    }

    pub fn pick_memorized_state(&mut self, id:u32) -> GenericNode<Config> {
        return self.memorized_states.remove(&id).unwrap();
    }

    pub fn remember_state(&mut self, id:u32, state : GenericNode<Config>) {
        assert!(!self.memorized_states.contains_key(&id));
        self.memorized_states.insert( id, state );
    }

    pub fn extract_from_queue(&mut self) -> Option<GenericStep<Config>> {
        match self.process_queue.dequeue() {
            None => {
                return None;
            },
            Some( (step,_) ) => {
                return Some(step);
            }
        }
    }

    pub fn apply_filters(&self,
                         depth : u32,
                         node_counter : u32,
                         criterion : &Config::FilterCriterion) -> Option<Config::FilterEliminationKind> {
        for filter in &self.filters {
            match filter.apply_filter(depth,node_counter,criterion) {
                None => {},
                Some( elim_kind) => {
                    return Some(elim_kind);
                }
            }
        }
        return None;
    }

    pub fn queue_set_last_reached_has_no_child(&mut self) {
        self.process_queue.set_last_reached_has_no_child();
    }

    fn reorganize_by_priority(
        priorities : &Config::Priorities,
        steps : Vec<GenericStep<Config>>,
        randomize : bool) -> Vec<GenericStep<Config>> {
        let mut reorganized : Vec<GenericStep<Config>> = vec![];
        {
            let mut by_priorities : HashMap<i32,Vec<GenericStep<Config>>> = hashmap!{};
            for child in steps {
                let priority = child.kind.get_priority(priorities);
                // ***
                match by_priorities.get_mut(&priority) {
                    None => {
                        by_priorities.insert(priority,vec![ child ]);
                    },
                    Some( at_priority ) => {
                        at_priority.push(child );
                    }
                }
            }
            // ***
            let mut keys : Vec<i32> = by_priorities.keys().cloned().collect();
            keys.sort();
            for k in keys {
                match by_priorities.get_mut(&k) {
                    None => {},
                    Some( queue ) => {
                        if randomize {
                            queue.shuffle(&mut thread_rng());
                        }
                        reorganized.append( queue );
                    }
                }
            }
        }
        return reorganized;
    }

    pub fn enqueue_new_steps(&mut self,
                             parent_id : u32,
                             to_enqueue : Vec<GenericStep<Config>>) {
        let reorganized = Self::reorganize_by_priority(&self.priorities.specific,
                                                 to_enqueue,
                                                 self.priorities.randomize);
        self.process_queue.enqueue(parent_id,reorganized);
    }

    /*
    pub fn get_basic_options_as_strings(&self) -> Vec<String> {
        let mut options_str : Vec<String> = Vec::new();
        options_str.push( format!("strategy={}", &self.strategy.to_string()) );
        options_str.push( format!("priorities={}", &self.priorities.to_string()) );
        {
            let mut filters_strs : Vec<String> = self.filters.iter()
                .map(|f| f.to_string()).collect();
            options_str.push( format!("filters=[{}]", filters_strs.join(",")) );
        }
        return options_str;
    }*/
}





