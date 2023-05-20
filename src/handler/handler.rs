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

pub trait AbstractProcessHandler<Conf : AbstractProcessConfiguration> {

    fn process_new_step(context : &Conf::Context,
                        param : &Conf::Parameterization,
                        parent_node : &GenericNode<Conf::NodeKind>,
                        step_to_process : &GenericStep<Conf::StepKind>,
                        new_node_id : u32,
                        node_counter : u32) -> Conf::NodeKind;

    fn get_criterion(context : &Conf::Context,
                     param : &Conf::Parameterization,
                     parent_node : &GenericNode<Conf::NodeKind>,
                     step_to_process : &GenericStep<Conf::StepKind>,
                     new_node_id : u32,
                     node_counter : u32) -> Conf::FilterCriterion;

    fn collect_next_steps(context : &Conf::Context,
                          param : &Conf::Parameterization,
                          parent_node_kind : &Conf::NodeKind) -> Vec<Conf::StepKind>;

    fn get_local_verdict_when_no_child(context : &Conf::Context,
                                       param : &Conf::Parameterization,
                                       node_kind : &Conf::NodeKind) -> Conf::LocalVerdict;

    fn get_local_verdict_from_static_analysis(context : &Conf::Context,
                                              param : &Conf::Parameterization,
                                              node_kind : &mut Conf::NodeKind)
            -> Option<(Conf::LocalVerdict,Conf::StaticLocalVerdictAnalysisProof)>;

    fn pursue_process_after_static_verdict(context : &Conf::Context,
                                           param : &Conf::Parameterization,
                                           loc_verd : &Conf::LocalVerdict) -> bool;

}

