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


use crate::config::AbstractConfiguration;
use crate::delegate::queue::generic::GenericProcessQueue;
use crate::delegate::queue::strategy::QueueSearchStrategy;

use crate::delegate::queue::q_bfs::BFS_ProcessQueue;
use crate::delegate::queue::q_dfs::DFS_ProcessQueue;
use crate::delegate::queue::q_hcs::HCS_ProcessQueue;

pub(crate) fn create_process_queue<Config : AbstractConfiguration + 'static>(strategy : &QueueSearchStrategy)
            -> Box< dyn GenericProcessQueue<Config> > {
    match strategy {
        QueueSearchStrategy::BFS => {
            return Box::new(BFS_ProcessQueue::<Config>::new() );
        },
        QueueSearchStrategy::DFS => {
            return Box::new(DFS_ProcessQueue::<Config>::new() );
        },
        QueueSearchStrategy::HCS => {
            return Box::new(HCS_ProcessQueue::<Config>::new() );
        }
    }
}

