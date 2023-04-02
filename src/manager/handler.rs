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
use crate::node::GenericNode;
use crate::step::GenericStep;

pub trait AbstractProcessHandler<Config:AbstractConfiguration> {

    fn process_new_step(parent_state : &GenericNode<Config>,
                        step_to_process   : &GenericStep<Config>,
                        new_state_id : u32,
                        node_counter : u32) -> Result<Config::NodeKind,Config::FilterEliminationKind>;


    fn collect_next_steps(parent_state_id : u32,
                          parent_node_kind : &Config::NodeKind) -> (u32,Vec<GenericStep<Config>>);

    fn get_local_verdict(context : &Config::ProcessContext, node : &Config::NodeKind) -> Config::LocalVerdict;

}





