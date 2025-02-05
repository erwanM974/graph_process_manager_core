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

use super::config::AbstractProcessConfiguration;



/** 
 * The process may have a global state that evolves as the graph structure is explored.
 * Typically, this may coïncide with:
 * - the fact that we have found an irreducible term in term rewriting
 *   (once this is the case, we may terminate the process)
 * - a global verdict for a runtime analysis algorithm
 * - etc.
 * **/
pub trait AbstractProcessMutablePersistentState<Conf : AbstractProcessConfiguration> {

    /** 
     * Initializes the global state at the start of the process.
     * **/
    fn get_initial_state(
        context_and_param : &Conf::ContextAndParameterization
    ) -> Self;

    /** 
     * Updates the global state once a node is reached.
     * **/
    fn update_on_node_reached(
        &mut self, 
        context_and_param : &Conf::ContextAndParameterization,
        node : &Conf::DomainSpecificNode
    );

    /** 
     * Updates the global state once, after a node is reached, its next steps are collected.
     * **/
    fn update_on_next_steps_collected_reached(
        &mut self, 
        context_and_param : &Conf::ContextAndParameterization,
        node : &Conf::DomainSpecificNode,
        steps : &[Conf::DomainSpecificStep]
    );


    fn update_on_filtered(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        parent_node : &Conf::DomainSpecificNode,
        filtration_result : &Conf::FiltrationResult
    );

    /** 
     * Returns true if the current global state warrants stopping the whole process.
     * Typically this could mean:
     * - in a rewrite system, that we have found an irreducible term
     * - in an offline runtime verification algorithm that we have reached a certain verdict
     * - etc.
     * **/
    fn warrants_termination_of_the_process(
        &self, 
        context_and_param : &Conf::ContextAndParameterization
    ) -> bool;

}