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


use crate::config::{AbstractConfiguration, AbstractStepKind};
use crate::step::GenericStep;




pub struct GenericProcessPriorities<Config : AbstractConfiguration> {
    pub specific : Config::Priorities,
    pub randomize : bool
}

impl <Config : AbstractConfiguration> GenericProcessPriorities<Config> {
    pub fn new(specific : Config::Priorities,
               randomize : bool) -> GenericProcessPriorities<Config> {
        return GenericProcessPriorities{specific,randomize};
    }
}

impl<Config : AbstractConfiguration> std::string::ToString for GenericProcessPriorities<Config> {
    fn to_string(&self) -> String {
        if self.randomize {
            return format!("randomize {}", self.specific.to_string());
        } else {
            return self.specific.to_string();
        }
    }
}
