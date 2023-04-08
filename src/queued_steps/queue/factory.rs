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

use crate::queued_steps::queue::generic::GenericProcessQueue;
use crate::queued_steps::queue::strategy::QueueSearchStrategy;

use crate::queued_steps::queue::q_bfs::BfsProcessQueue;
use crate::queued_steps::queue::q_dfs::DfsProcessQueue;
use crate::queued_steps::queue::q_hcs::HcsProcessQueue;

pub fn create_process_queue<T : 'static>(strategy : &QueueSearchStrategy)
        -> Box< dyn GenericProcessQueue<T> > {
    match strategy {
        QueueSearchStrategy::BFS => {
            Box::new(BfsProcessQueue::<T>::new() )
        },
        QueueSearchStrategy::DFS => {
            Box::new(DfsProcessQueue::<T>::new() )
        },
        QueueSearchStrategy::HCS => {
            Box::new(HcsProcessQueue::<T>::new() )
        }
    }
}

