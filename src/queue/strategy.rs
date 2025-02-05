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


use std::fmt;

use super::queue_kinds::{generic::AbstractStepsQueue, q_bfs::BfsStepsQueue, q_dfs::DfsStepsQueue, q_hcs::HcsStepsQueue};


pub enum QueueSearchStrategy {
    BFS, // breadth first search
    DFS, // depth first search
    HCS  // high coverage search
}

impl fmt::Display for QueueSearchStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueueSearchStrategy::BFS => {
                write!(f,"BreadthFirstSearch")
            },
            QueueSearchStrategy::DFS => {
                write!(f,"DepthFirstSearch")
            },
            QueueSearchStrategy::HCS => {
                write!(f,"HighCoverageSearch")
            }
        }
    }
}

impl QueueSearchStrategy {

    pub(in crate::queue) fn create_process_queue<DomainSpecificStep : 'static>(&self) -> Box< dyn AbstractStepsQueue<DomainSpecificStep> > {
        match self {
            QueueSearchStrategy::BFS => {
                Box::new(BfsStepsQueue::<DomainSpecificStep>::new() )
            },
            QueueSearchStrategy::DFS => {
                Box::new(DfsStepsQueue::<DomainSpecificStep>::new() )
            },
            QueueSearchStrategy::HCS => {
                Box::new(HcsStepsQueue::<DomainSpecificStep>::new() )
            }
        }
    }

}