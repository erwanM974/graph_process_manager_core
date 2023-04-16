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



use crate::delegate::node::GenericNode;
use crate::manager::config::AbstractProcessConfiguration;
use crate::queued_steps::step::GenericStep;

pub trait AbstractProcessHandler<Config : AbstractProcessConfiguration> {

    fn process_new_step(context : &Config::Context,
                        param : &Config::Parameterization,
                        parent_node : &GenericNode<Config::NodeKind>,
                        step_to_process : &GenericStep<Config::StepKind>,
                        new_node_id : u32,
                        node_counter : u32) -> Config::NodeKind;

    fn get_criterion(context : &Config::Context,
                     param : &Config::Parameterization,
                     parent_node : &GenericNode<Config::NodeKind>,
                     step_to_process : &GenericStep<Config::StepKind>,
                     new_node_id : u32,
                     node_counter : u32) -> Config::FilterCriterion;

    fn collect_next_steps(context : &Config::Context,
                          param : &Config::Parameterization,
                          parent_node_id : u32,
                          parent_node_kind : &Config::NodeKind) -> (u32,Vec<GenericStep<Config::StepKind>>);

    fn get_local_verdict_when_no_child(context : &Config::Context,
                                       param : &Config::Parameterization,
                                       node_kind : &Config::NodeKind) -> Config::LocalVerdict;

    fn get_local_verdict_from_static_analysis(context : &Config::Context,
                                              param : &Config::Parameterization,
                                              node_kind : &Config::NodeKind) -> Config::LocalVerdict;

    fn pursue_process_after_static_verdict(context : &Config::Context,
                                           param : &Config::Parameterization,
                                           loc_verd : &Config::LocalVerdict) -> bool;

}

