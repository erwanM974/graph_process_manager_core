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



use crate::process::config::AbstractProcessConfiguration;

pub trait AbstractAlgorithmOperationHandler<Conf : AbstractProcessConfiguration> {

    fn process_new_step(
        context_and_param : &Conf::ContextAndParameterization,
        global_state : &mut Conf::MutablePersistentState,
        parent_node : &Conf::DomainSpecificNode,
        step_to_process : &mut Conf::DomainSpecificStep
    ) -> Conf::DomainSpecificNode;

    fn collect_next_steps(
        context_and_param : &Conf::ContextAndParameterization,
        global_state : &mut Conf::MutablePersistentState,
        parent_node : &Conf::DomainSpecificNode
    ) -> Vec<Conf::DomainSpecificStep>;

}

