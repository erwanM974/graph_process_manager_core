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
use rand::rng;

use crate::{process::config::AbstractNodeKind, queue::{memorized_node::MemorizedNode, queue_kinds::generic::AbstractStepsQueue, queued_step::EnqueuedStep, strategy::QueueSearchStrategy}};

use super::priorities::{AbstractPriorities, GenericProcessPriorities};



/** 
 * The process queue delegate is tasked with:
 * - 
 * **/
pub(crate) struct ProcessQueueDelegate<
    DomainSpecificStep,
    DomainSpecificNode : AbstractNodeKind,
    Priorities : AbstractPriorities<DomainSpecificStep>
> {
    strategy : QueueSearchStrategy,
    priorities : GenericProcessPriorities<Priorities>,
    memorized_nodes : HashMap<u32,MemorizedNode<DomainSpecificNode>>,
    process_queue : Box< dyn AbstractStepsQueue<DomainSpecificStep> >
}

impl<
    DomainSpecificStep : 'static, 
    DomainSpecificNode : AbstractNodeKind, 
    Priorities: AbstractPriorities<DomainSpecificStep>
> ProcessQueueDelegate<DomainSpecificStep, DomainSpecificNode, Priorities> {
    pub fn new(strategy: QueueSearchStrategy,
               priorities: GenericProcessPriorities<Priorities>) -> Self {
        let process_queue = strategy.create_process_queue();
        ProcessQueueDelegate{
            strategy,
            priorities,
            memorized_nodes:hashmap!{},
            process_queue}
    }

    pub fn get_strategy(&self) -> &QueueSearchStrategy {
        &self.strategy
    }

    pub fn get_priorities(&self) -> &GenericProcessPriorities<Priorities> {
        &self.priorities
    }

    pub fn get_mut_memorized_node(&mut self, id:u32) -> &mut MemorizedNode<DomainSpecificNode> {
        self.memorized_nodes.get_mut(&id).unwrap()
    }

    pub fn get_memorized_node(&self, id:u32) -> &MemorizedNode<DomainSpecificNode> {
        self.memorized_nodes.get(&id).unwrap()
    }

    pub fn extract_from_queue(&mut self) -> Option<(EnqueuedStep<DomainSpecificStep>,Option<MemorizedNode<DomainSpecificNode>>)> {
        if let Some((step,parent_has_no_more_child)) = self.process_queue.dequeue() {
            if let Some(parent_node_id) = parent_has_no_more_child {
                let x = self.memorized_nodes.remove(&parent_node_id).unwrap();
                Some((step,Some(x)))
            } else {
                Some((step,None))
            }
        } else {
            None 
        }
    }

    pub fn queue_set_last_reached_has_no_child(&mut self) {
        self.process_queue.set_last_reached_has_no_child();
    }

    pub fn enqueue_new_steps(
        &mut self,
        parent_node : MemorizedNode<DomainSpecificNode>,
        parent_node_id : u32,
        child_steps_to_enqueue : Vec<EnqueuedStep<DomainSpecificStep>>) 
    {
        assert!(!self.memorized_nodes.contains_key(&parent_node_id));
        self.memorized_nodes.insert( parent_node_id, parent_node );
        // ***
        assert!(!child_steps_to_enqueue.is_empty());
        let reorganized = Self::reorganize_by_priority(
            &self.priorities.domain_specific,
            child_steps_to_enqueue,
            self.priorities.randomize
        );
        self.process_queue.enqueue(parent_node_id,reorganized);
    }

    fn reorganize_by_priority(
        priorities : &Priorities,
        steps : Vec<EnqueuedStep<DomainSpecificStep>>,
        randomize : bool) -> Vec<EnqueuedStep<DomainSpecificStep>> {
        let mut reorganized : Vec<EnqueuedStep<DomainSpecificStep>> = vec![];
        {
            let mut by_priorities : HashMap<i32,Vec<EnqueuedStep<DomainSpecificStep>>> = hashmap!{};
            for child in steps {
                let priority = priorities.get_priority_of_step(&child.domain_specific_step);
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
                            queue.shuffle(&mut rng());
                        }
                        reorganized.append( queue );
                    }
                }
            }
        }
        // ***
        reorganized
    }

}




