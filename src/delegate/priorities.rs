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





pub trait AbstractPriorities<Step> : Sized + std::string::ToString {

    fn get_priority_of_step(&self, step : &Step) -> i32;

}

pub struct GenericProcessPriorities<Priorities : std::string::ToString> {
    pub specific : Priorities,
    pub randomize : bool
}

impl<Priorities : std::string::ToString> std::string::ToString for GenericProcessPriorities<Priorities> {
    fn to_string(&self) -> String {
        if self.randomize {
            format!("randomize {}", self.specific.to_string())
        } else {
            self.specific.to_string()
        }
    }
}


impl<Priorities : std::string::ToString> GenericProcessPriorities<Priorities> {
    pub fn new(specific: Priorities, randomize: bool) -> Self {
        GenericProcessPriorities { specific, randomize }
    }
}
