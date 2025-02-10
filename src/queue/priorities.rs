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


/** 
 * A trait to implement custom prioritization in the order of evaluation of the next steps that may be taken from the same node.
 * **/
pub trait AbstractPriorities<DomainSpecificStep> : Sized {

    /** 
     * Returns an integer score that indicate the prioirity of a specific step.
     * **/
    fn get_priority_of_step(&self, domain_specific_step : &DomainSpecificStep) -> i32;

    fn get_description(&self) -> Vec<String>;
    
}

/** 
 * Priorities of the next steps that may be taken from the same node are customized as follows:
 * - for each step, we may compute a priority score given as an integer value
 * - the steps are then reordered as per these scores
 * - for steps with the same score, we may or may not shuffle them randomly
 * **/
pub struct GenericProcessPriorities<Priorities> {
    pub domain_specific : Priorities,
    pub randomize : bool
}




impl<Priorities> GenericProcessPriorities<Priorities> {
    pub fn new(domain_specific: Priorities, randomize: bool) -> Self {
        GenericProcessPriorities { domain_specific, randomize }
    }
}
