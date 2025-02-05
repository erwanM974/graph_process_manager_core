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

use crate::queue::queued_step::EnqueuedStep;



/** 
 * A queue to enqueue and dequeue steps in a process.
 * **/
pub trait AbstractStepsQueue<DomainSpecificStep> {

    fn new() -> Self where Self : Sized;

    /** 
     * Returns a next step to execute in the process.
     * 
     * If the parent node from which this step is fired has no other child left 
     * (i.e., there are no, not yet fired steps from it)
     * then return its ID so that we may then forget it / erase from memory
    **/
    fn dequeue(&mut self) -> Option<(EnqueuedStep<DomainSpecificStep>,Option<u32>)>;


    /** 
     * Enqueue all the potential next steps (prealably intentionally ordered) that may be taken from a given node.
    **/
    fn enqueue(&mut self,
               parent_node_id : u32,
               to_enqueue : Vec<EnqueuedStep<DomainSpecificStep>>);

    
    fn set_last_reached_has_no_child(&mut self);

}