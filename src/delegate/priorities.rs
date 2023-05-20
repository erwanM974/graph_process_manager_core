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

pub trait AbstractPriorities<Step> : Sized + fmt::Display {

    fn get_priority_of_step(&self, step : &Step) -> i32;

}

pub struct GenericProcessPriorities<Priorities : fmt::Display> {
    pub specific : Priorities,
    pub randomize : bool
}

impl<Priorities : fmt::Display> fmt::Display for GenericProcessPriorities<Priorities> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.randomize {
            write!(f,"randomize {}", self.specific)
        } else {
            write!(f,"{}", self.specific)
        }
    }
}


impl<Priorities : fmt::Display> GenericProcessPriorities<Priorities> {
    pub fn new(specific: Priorities, randomize: bool) -> Self {
        GenericProcessPriorities { specific, randomize }
    }
}
