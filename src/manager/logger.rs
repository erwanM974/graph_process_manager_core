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


use crate::delegate::priorities::GenericProcessPriorities;
use crate::handler::filter::AbstractFilter;
use crate::manager::config::AbstractProcessConfiguration;
use crate::queued_steps::queue::strategy::QueueSearchStrategy;

pub trait AbstractProcessLogger<Conf : AbstractProcessConfiguration> {

    fn log_initialize(&mut self);

    fn log_parameterization(&mut self,
                            strategy : &QueueSearchStrategy,
                            filters : &[Box<dyn AbstractFilter<Conf::FilterCriterion,Conf::FilterEliminationKind>>],
                            priorities : &GenericProcessPriorities<Conf::Priorities>,
                            parameterization : &Conf::Parameterization);

    fn log_filtered(&mut self,
                    context : &Conf::Context,
                    parent_node_id : u32,
                    new_node_id : u32,
                    elim_kind : &Conf::FilterEliminationKind);

    fn log_new_node(&mut self,
                    context : &Conf::Context,
                    param : &Conf::Parameterization,
                    new_node_id : u32,
                    new_node : &Conf::NodeKind);

    fn log_new_step(&mut self,
                    context : &Conf::Context,
                    param : &Conf::Parameterization,
                    origin_node_id : u32,
                    target_node_id : u32,
                    step : &Conf::StepKind,
                    target_node : &Conf::NodeKind,
                    target_depth : u32);

    fn log_verdict_on_no_child(&mut self,
                               context : &Conf::Context,
                               param : &Conf::Parameterization,
                               parent_node_id : u32,
                               verdict : &Conf::LocalVerdict);

    fn log_verdict_on_static_analysis(&mut self,
                                      context : &Conf::Context,
                                      param : &Conf::Parameterization,
                                      parent_node_id : u32,
                                      verdict : &Conf::LocalVerdict,
                                      proof : &Conf::StaticLocalVerdictAnalysisProof);

    fn log_terminate(&mut self,
                     global_verdict : &Conf::GlobalVerdict);

    fn log_notify_terminal_node_reached(&mut self,
                                        context : &Conf::Context,
                                        node_id : u32);

    fn log_notify_last_child_of_node_processed(&mut self,
                                               context : &Conf::Context,
                                               parent_node_id : u32);

}