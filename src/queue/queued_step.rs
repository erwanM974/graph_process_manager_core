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
 * Our process is performed by enqueuing the next steps that may result from the evaluation of a node.
 * These steps are domain specific and taking on such step may yield another node, which in turn may yield another set of potential next steps and so on.
 * 
 * This struct encodes one such step.
 * It carries the domain specific nature of the step
 * **/
pub struct EnqueuedStep<DomainSpecificStep> {
    pub parent_node_id : u32,
    pub id_as_potential_step_from_parent : u32,
    pub domain_specific_step : DomainSpecificStep
}

impl<DomainSpecificStep> EnqueuedStep<DomainSpecificStep> {
    pub fn new(
        parent_node_id : u32,
        id_as_potential_step_from_parent : u32,
        domain_specific_step : DomainSpecificStep
    ) -> Self {
        Self{
            parent_node_id,
            id_as_potential_step_from_parent,
            domain_specific_step
        }
    }
}
