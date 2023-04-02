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
use std::hash::Hash;


use crate::delegate::filter::AbstractFilter;
use crate::logger::AbstractProcessLogger;
use crate::manager::handler::AbstractProcessHandler;
use crate::manager::verdict::AbstractGlobalVerdict;


pub trait AbstractConfiguration : Sized {
    type ProcessContext;
    type ProcessParameterization;
    // ***
    type NodeKind : AbstractNodeKind;
    type StepKind : AbstractStepKind<Self>;
    type Priorities : AbstractPriorities;
    // ***
    type Filter : AbstractFilter<Self>;
    type FilterCriterion;
    type FilterEliminationKind : std::string::ToString;
    // ***
    type Logger : AbstractProcessLogger<Self>;
    type GlobalVerdict : AbstractGlobalVerdict<Self>;
    type LocalVerdict;
    // ***
    type ProcessHandler : AbstractProcessHandler<Self>;
}


pub trait AbstractPriorities : Sized + std::string::ToString {}

pub trait AbstractStepKind<Config : AbstractConfiguration> : Sized {

    fn get_priority(&self, process_priorities : &Config::Priorities) -> i32;

}



pub trait AbstractNodeKind : Sized + Clone + PartialEq + Eq + Hash {

    fn is_included_for_memoization(&self, memoized_node : &Self) -> bool;

}