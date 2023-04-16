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


use std::hash::Hash;
use crate::delegate::priorities::AbstractPriorities;
use crate::handler::handler::AbstractProcessHandler;
use crate::manager::verdict::AbstractGlobalVerdict;

pub trait AbstractProcessConfiguration : Sized {
    type Context;
    type Parameterization : AbstractProcessParameterization;
    // ***
    type NodeKind : AbstractNodeKind;
    type StepKind;
    type Priorities : AbstractPriorities<Self::StepKind>;
    // ***
    type FilterCriterion : std::string::ToString;
    type FilterEliminationKind : std::string::ToString;
    // ***
    type LocalVerdict : std::string::ToString;
    type GlobalVerdict : AbstractGlobalVerdict<Self::LocalVerdict>;
    // ***
    type ProcessHandler : AbstractProcessHandler<Self>;
}



pub trait AbstractNodeKind : Sized + Clone + PartialEq + Eq + Hash {

    fn is_included_for_memoization(&self, memoized_node : &Self) -> bool;

}

pub trait AbstractProcessParameterization {

    fn get_param_as_strings(&self) -> Vec<String>;

}