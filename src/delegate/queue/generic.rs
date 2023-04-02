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
use crate::step::GenericStep;

pub trait GenericProcessQueue<Config : AbstractConfiguration> {

    fn new() -> Self where Self : Sized;

    /** returns a next step to execute
       and if the parent state from which this step is fired
       has no other child left
       then return its ID
       so that we may then forget it / erase from memory
          **/
    fn dequeue(&mut self) -> Option<(GenericStep<Config>,Option<u32>)>;

    fn enqueue(&mut self,
               parent_id : u32,
               to_enqueue : Vec<GenericStep<Config>>);

    fn set_last_reached_has_no_child(&mut self);

}