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

use super::config::{AbstractNodeKind, AbstractProcessConfiguration};



pub(crate) enum NodeMemoizer<Conf : AbstractProcessConfiguration> {
    Memoizing(HashMap<Conf::DomainSpecificNode,u32>),
    NotMemoizing
}

impl<Conf : AbstractProcessConfiguration> NodeMemoizer<Conf> {

    pub fn new(is_memoized : bool) -> Self {
        if is_memoized {
            Self::Memoizing(hashmap!{})
        } else {
            Self::NotMemoizing
        }
    }

    pub fn is_memoized(&self) -> bool {
        match &self {
            NodeMemoizer::Memoizing(_) => true,
            NodeMemoizer::NotMemoizing => false
        }
    }

    pub fn check_memo(&self, to_look_up : &Conf::DomainSpecificNode) -> Option<u32> {
        match &self {
            NodeMemoizer::Memoizing(memo) => {
                for (memoized_node, memoized_node_id) in memo {
                    if to_look_up.is_included_for_memoization(memoized_node) {
                        return Some(*memoized_node_id);
                    }
                }
                None
            },
            NodeMemoizer::NotMemoizing => {
                None 
            }
        }
    }

    pub fn memoize_new_node(&mut self, new_node : &Conf::DomainSpecificNode, new_node_id : u32) {
        match self {
            NodeMemoizer::Memoizing(memo) => {
                memo.insert(new_node.clone(),new_node_id);
            },
            NodeMemoizer::NotMemoizing => {}
        }
    }

}
