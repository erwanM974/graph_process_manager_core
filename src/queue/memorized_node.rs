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

use crate::process::config::AbstractNodeKind;



/** 
 * Our process is performed by enqueuing the next steps that may result from the evaluation of a node.
 * Taking on such step may yield another node, which in turn may yield another set of potential next steps and so on.
 * 
 * This struct encodes one such node.
 * It carries the domain specific nature of the node.
 * And keeps track of the identifiers of which steps that may be fired from it are yet to be processed.
 * **/
 #[derive(Clone, PartialEq, Eq)]
pub struct MemorizedNode<DomainSpecificNode : AbstractNodeKind> {
    pub domain_specific_node : DomainSpecificNode,
    pub remaining_child_steps_ids_to_process : HashSet<u32>
}

impl<DomainSpecificNode : AbstractNodeKind> MemorizedNode<DomainSpecificNode> {
    pub fn new(domain_specific_node: DomainSpecificNode, remaining_child_steps_ids_to_process: HashSet<u32>) -> Self {
        Self { 
            domain_specific_node, 
            remaining_child_steps_ids_to_process
        }
    }
}

