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


use std::collections::VecDeque;

use crate::config::AbstractConfiguration;
use crate::step::GenericStep;
use crate::delegate::queue::generic::GenericProcessQueue;


pub struct BFS_ProcessQueue<Config : AbstractConfiguration> {
    queue : VecDeque< (u32,Vec<GenericStep<Config>>) >
}

impl<Config : AbstractConfiguration> GenericProcessQueue<Config> for BFS_ProcessQueue<Config> {

    fn new() -> BFS_ProcessQueue<Config> {
        return BFS_ProcessQueue{queue:VecDeque::new()};
    }

    fn dequeue(&mut self) -> Option<(GenericStep<Config>,Option<u32>)> {
        match self.queue.pop_front() {
            None => {
                return None;
            },
            Some( (parent_id,mut rem) ) => {
                match rem.pop() {
                    None => {
                        panic!("should never have an empty vector here");
                    },
                    Some( got_step ) => {
                        if rem.len() > 0 {
                            self.queue.push_front((parent_id,rem) );
                            return Some( (got_step,None) );
                        } else {
                            return Some( (got_step,Some(parent_id)) );
                        }
                    }
                }
            }
        }
    }

    fn enqueue(&mut self,
               parent_id : u32,
               to_enqueue : Vec<GenericStep<Config>>) {
        if to_enqueue.len() > 0 {
            self.queue.push_back( (parent_id,to_enqueue) );
        }
    }

    fn set_last_reached_has_no_child(&mut self) {}

}
