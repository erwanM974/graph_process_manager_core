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

use crate::queue::priorities::AbstractPriorities;

use super::{handler::AbstractAlgorithmOperationHandler, persistent_state::AbstractProcessMutablePersistentState};



/** 
 * Configuration of an abstract process that performs a certain computation.
 * This can be, for instance:
 * - a term rewrite system
 * - a runtime verification algorithm
 * - ...
 * 
 * The main thing in common with the kinds of processes that can be modelled this way is
 * that they can be represented as a search in a graph structure 
 * (which may be finite or not, and which is not known in advance).
 * 
 * The graph structure is not known in advance. At the start of the process, we only know of a single node which is the initial node.
 * From any given node, a number of distinct possible steps may be taken.
 * Each such step may yield to the discovery of a new node.
 * Thus, the graph structure is explored.
 * 
 * The overall process is characterized by:
 * - the problem which it tries to solve, this problem being represented by:
 *   + a "Context"
 *   + and the initial node of the graph from which the search starts
 * - the algorithm that solves this problem, this algorithm being represented by:
 *   + a "Parameterization" which allows encoding variants of that algorithm without code duplication
 *   + an "AlgorithmOperationHandler" that is tasked with performing the following operations:
 *     * compute a child node from the evaluation of a step from a parent node
 *     * collect all possible next steps that could be taken after reaching a given node
 *     * etc
 *     * see [AbstractAlgorithmOperationHandler](AbstractAlgorithmOperationHandler) for details
 * 
 * Because the "Context" and "Parameterization" are often entertwined in practice, we represent
 * them together in the "ContextAndParameterization" associated type.
 * 
 * Both nodes and steps carry domain-specific information.
 * This information is encoded via the associate types:
 * - "DomainSpecificNode"
 * - "DomainSpecificStep"
 * 
 * Priorities in the order of evaluations of the steps that may be taken from the same node
 * can be configured via the "Priorities" associated type.
 * 
 * The process has a global state that may evolve as the graph structure is evolved.
 * This is represented by the "MutablePersistentState" associated type.
 * 
 * We consider three kinds of filters:
 * - NodesPreFilters, which evaluate newly encountered nodes
 * - NodesPostFilters, which evalute the set of steps that may be taken from a newly encountered node
 * - StepsFilters, which evalute individual steps
 * 
 * Any of these filters may be used to stop/prevent/preempt the exploration of parts of the graph structure.
 * These filters return an Option<FiltrationResult>:
 * - if is is None, then the node or step is evaluated normally
 * - if is is Some(x), then the process do not explore the successors of the filtered node/step
 *   and x is further used to change the global state and notify loggers
 * **/
pub trait AbstractProcessConfiguration : Sized {
    // ***
    type ContextAndParameterization : AbstractContextAndParameterization;
    type AlgorithmOperationHandler : AbstractAlgorithmOperationHandler<Self>;
    // ***
    type DomainSpecificNode : AbstractNodeKind;
    type DomainSpecificStep;
    type Priorities : AbstractPriorities<Self::DomainSpecificStep>;
    // ***
    type MutablePersistentState : AbstractProcessMutablePersistentState<Self>;
    // ***
    type FiltrationResult;
    // ***
}



pub trait AbstractNodeKind : Sized + Clone + PartialEq + Eq + Hash {

    fn is_included_for_memoization(&self, memoized_node : &Self) -> bool;

}

pub trait AbstractContextAndParameterization {

    /** 
     * Returns a title describing the whole process.
     * **/
    fn get_process_description(&self) -> &str;

    /** 
     * Returns the description of individual relevant parameters.
     * **/
    fn get_parameters_description(&self) -> Vec<&str>;

}