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
use crate::delegate::priorities::GenericProcessPriorities;
use crate::delegate::queue::strategy::QueueSearchStrategy;

pub trait AbstractProcessLogger<Config : AbstractConfiguration> {

    fn log_initialize(&mut self,
                context : &Config::ProcessContext,
                init_state_id : u32,
                init_node : &Config::NodeKind);

    fn log_parameterization(&mut self,
                            strategy : &QueueSearchStrategy,
                            filters : &Vec<Config::Filter>,
                            priorities : &GenericProcessPriorities<Config>,
                            process_parameterization : &Config::ProcessParameterization);

    fn log_filtered(&mut self,
                    context : &Config::ProcessContext,
                    parent_state_id : u32,
                    new_state_id : u32,
                    elim_kind : &Config::FilterEliminationKind);

    fn log_new_node(&mut self,
                    context : &Config::ProcessContext,
                    new_state_id : u32,
                    new_node : &Config::NodeKind);

    fn log_new_transition(&mut self,
                    context : &Config::ProcessContext,
                    origin_state_id : u32,
                    target_state_id : u32,
                    step : &Config::StepKind);

    fn log_verdict(&mut self,
                   context : &Config::ProcessContext,
                   parent_state_id : u32,
                   verdict : &Config::LocalVerdict);

    fn log_terminate(&mut self,
                     global_verdict : &Config::GlobalVerdict);

    fn log_notify_terminal_node_reached(&mut self,
                                        context : &Config::ProcessContext,
                                        node_id : u32);

    fn log_notify_last_child_of_node_processed(&mut self,
                                        context : &Config::ProcessContext,
                                        parent_node_id : u32);

}