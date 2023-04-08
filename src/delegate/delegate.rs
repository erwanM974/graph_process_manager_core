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

use crate::delegate::node::GenericNode;

use crate::delegate::priorities::{AbstractPriorities, GenericProcessPriorities};
use crate::queued_steps::queue::factory::create_process_queue;
use crate::queued_steps::queue::generic::GenericProcessQueue;
use crate::queued_steps::queue::strategy::QueueSearchStrategy;
use crate::queued_steps::step::GenericStep;


pub struct GenericProcessDelegate<Step,Node,Priorities : AbstractPriorities<Step>> {
    strategy : QueueSearchStrategy,
    priorities : GenericProcessPriorities<Priorities>,
    memorized_states : HashMap<u32,GenericNode<Node>>,
    process_queue : Box< dyn GenericProcessQueue<Step> >
}

impl<Step : 'static, Node, Priorities: AbstractPriorities<Step>> GenericProcessDelegate<Step, Node, Priorities> {
    pub fn new(strategy: QueueSearchStrategy,
               priorities: GenericProcessPriorities<Priorities>) -> Self {
        let process_queue = create_process_queue(&strategy);
        GenericProcessDelegate{
            strategy,
            priorities,
            memorized_states:hashmap!{},
            process_queue}
    }

    pub fn get_strategy(&self) -> &QueueSearchStrategy {
        &self.strategy
    }

    pub fn get_priorities(&self) -> &GenericProcessPriorities<Priorities> {
        &self.priorities
    }

    pub fn pick_memorized_state(&mut self, id:u32) -> GenericNode<Node> {
        self.memorized_states.remove(&id).unwrap()
    }

    pub fn remember_state(&mut self, id:u32, state : GenericNode<Node>) {
        assert!(!self.memorized_states.contains_key(&id));
        self.memorized_states.insert( id, state );
    }

    pub fn extract_from_queue(&mut self) -> Option<GenericStep<Step>> {
        self.process_queue.dequeue().map(|(step,_)| step)
    }

    pub fn queue_set_last_reached_has_no_child(&mut self) {
        self.process_queue.set_last_reached_has_no_child();
    }

    fn reorganize_by_priority(
        priorities : &Priorities,
        steps : Vec<GenericStep<Step>>,
        randomize : bool) -> Vec<GenericStep<Step>> {
        let mut reorganized : Vec<GenericStep<Step>> = vec![];
        {
            let mut by_priorities : HashMap<i32,Vec<GenericStep<Step>>> = hashmap!{};
            for child in steps {
                let priority = priorities.get_priority_of_step(&child.kind);
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
        // ***
        reorganized
    }

    pub fn enqueue_new_steps(&mut self,
                             parent_id : u32,
                             to_enqueue : Vec<GenericStep<Step>>) {
        let reorganized = Self::reorganize_by_priority(&self.priorities.specific,
                                                 to_enqueue,
                                                 self.priorities.randomize);
        self.process_queue.enqueue(parent_id,reorganized);
    }

}




