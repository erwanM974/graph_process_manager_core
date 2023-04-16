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

pub trait AbstractProcessLogger<Config : AbstractProcessConfiguration> {

    fn log_initialize(&mut self);

    fn log_parameterization(&mut self,
                            strategy : &QueueSearchStrategy,
                            filters : &[Box<dyn AbstractFilter<Config::FilterCriterion,Config::FilterEliminationKind>>],
                            priorities : &GenericProcessPriorities<Config::Priorities>,
                            parameterization : &Config::Parameterization);

    fn log_filtered(&mut self,
                    context : &Config::Context,
                    parent_node_id : u32,
                    new_node_id : u32,
                    elim_kind : &Config::FilterEliminationKind);

    fn log_new_node(&mut self,
                    context : &Config::Context,
                    param : &Config::Parameterization,
                    new_node_id : u32,
                    new_node : &Config::NodeKind);

    fn log_new_step(&mut self,
                    context : &Config::Context,
                    param : &Config::Parameterization,
                    origin_node_id : u32,
                    target_node_id : u32,
                    step : &Config::StepKind,
                    target_node : &Config::NodeKind);

    fn log_verdict(&mut self,
                   context : &Config::Context,
                   param : &Config::Parameterization,
                   parent_node_id : u32,
                   verdict : &Config::LocalVerdict);

    fn log_terminate(&mut self,
                     global_verdict : &Config::GlobalVerdict);

    fn log_notify_terminal_node_reached(&mut self,
                                        context : &Config::Context,
                                        node_id : u32);

    fn log_notify_last_child_of_node_processed(&mut self,
                                               context : &Config::Context,
                                               parent_node_id : u32);

}