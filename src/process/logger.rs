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

use std::{any::Any, slice::IterMut};

use crate::{process::config::AbstractProcessConfiguration, queue::{priorities::GenericProcessPriorities, strategy::QueueSearchStrategy}};

use super::filter::GenericFiltersManager;

pub trait AbstractProcessLogger<Conf : AbstractProcessConfiguration> {

    /** 
     * Used to downcast the abstract logger into any concrete type that implements it.
     * **/
    fn as_any(&self) -> &dyn Any;

    /** 
     * Initializes the logger.
     * 
     * Also provide extensive initial information about the process configuration
     * which may or may not be needed depending on the actual logger. 
     * **/
    fn log_initialize(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        strategy : &QueueSearchStrategy,
        priorities : &GenericProcessPriorities<Conf::Priorities>,
        filters_manager : &GenericFiltersManager<Conf>,
        initial_global_state : &Conf::MutablePersistentState,
        use_memoization : bool,
    );

    /** 
     * Notifies the logger that a new node has been encountered and added to the graph structure.
     * **/
    fn log_new_node(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        new_node_id : u32,
        new_node : &Conf::DomainSpecificNode
    );

    /** 
     * Notifies the logger that a new step has been processed.
     * This is done separately from "log_new_node" because
     * processing a new step does not necessarily cause a new node to be added.
     * Indeed, if memoization is used, it may cycle back to an already known node.
     * **/
    fn log_new_step(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        origin_node_id : u32,
        step : &Conf::DomainSpecificStep,
        target_node_id : u32
    );

    /** 
     * Notifies the logger that all steps that could be fired from a given node
     * have been processed.
     * **/
    fn log_notify_last_child_step_of_node_processed(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        parent_node_id : u32
    );

    /** 
     * Notifies the logger that the given node does not have any children.
     * Being notified of such terminal nodes is sometimes relevant.
     * **/
    fn log_notify_node_without_children(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        node_id : u32
    );

    /** 
     * Notifies the logger that a filter has yielded a "FiltrationResult"
     * and therefore prevented the exploration of parts of the graph structure
     * that correspond to successors of a given node (here identified by "parent_node_id").
     * **/
    fn log_filtered(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        parent_node_id : u32,
        filtration_result_id : u32,
        filtration_result : &Conf::FiltrationResult
    );

    /** 
     * Notifies the logger that the process has terminated.
     * Carries the information of the final global state.
     * **/
    fn log_terminate_process(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        global_state : &Conf::MutablePersistentState
    );

}




pub(crate) fn loggers_initialize<Conf : AbstractProcessConfiguration>(
    loggers_iter : IterMut<'_, Box< dyn AbstractProcessLogger<Conf>>>,
    context_and_param : &Conf::ContextAndParameterization,
    strategy : &QueueSearchStrategy,
    priorities : &GenericProcessPriorities<Conf::Priorities>,
    filters_manager : &GenericFiltersManager<Conf>,
    initial_global_state : &Conf::MutablePersistentState,
    use_memoization : bool
) {
    for logger in loggers_iter {
        logger.log_initialize(
            context_and_param,
            strategy,
            priorities,
            filters_manager,
            initial_global_state,
            use_memoization
        );
    }
}






pub(crate) fn loggers_new_node<Conf : AbstractProcessConfiguration>(
    loggers_iter : IterMut<'_, Box< dyn AbstractProcessLogger<Conf>>>,
    context_and_param : &Conf::ContextAndParameterization,
    new_node_id : u32,
    new_node : &Conf::DomainSpecificNode
) {
    for logger in loggers_iter {
        logger.log_new_node(
            context_and_param,
            new_node_id,
            new_node
        );
    }
}






pub(crate) fn loggers_new_step<Conf : AbstractProcessConfiguration>(
    loggers_iter : IterMut<'_, Box< dyn AbstractProcessLogger<Conf>>>,
    context_and_param : &Conf::ContextAndParameterization,
    origin_node_id : u32,
    step : &Conf::DomainSpecificStep,
    target_node_id : u32
) {
    for logger in loggers_iter {
        logger.log_new_step(
            context_and_param,
            origin_node_id,
            step,
            target_node_id
        );
    }
}





pub(crate) fn loggers_notify_last_child_step_of_node_processed<Conf : AbstractProcessConfiguration>(
    loggers_iter : IterMut<'_, Box< dyn AbstractProcessLogger<Conf>>>,
    context_and_param : &Conf::ContextAndParameterization,
    parent_node_id : u32
) {
    for logger in loggers_iter {
        logger.log_notify_last_child_step_of_node_processed(
            context_and_param,
            parent_node_id
        );
    }
}


pub(crate) fn loggers_notify_node_without_children<Conf : AbstractProcessConfiguration>(
    loggers_iter : IterMut<'_, Box< dyn AbstractProcessLogger<Conf>>>,
    context_and_param : &Conf::ContextAndParameterization,
    node_id : u32
) {
    for logger in loggers_iter {
        logger.log_notify_node_without_children(
            context_and_param,
            node_id
        );
    }
}



pub(crate) fn loggers_filtered<Conf : AbstractProcessConfiguration>(
    loggers_iter : IterMut<'_, Box< dyn AbstractProcessLogger<Conf>>>,
    context_and_param : &Conf::ContextAndParameterization,
    parent_node_id : u32,
    filtration_result_id : u32,
    filtration_result : &Conf::FiltrationResult
) {
    for logger in loggers_iter {
        logger.log_filtered(
            context_and_param,
            parent_node_id,
            filtration_result_id,
            filtration_result
        );
    }
}





pub(crate) fn loggers_terminate_process<Conf : AbstractProcessConfiguration>(
    loggers_iter : IterMut<'_, Box< dyn AbstractProcessLogger<Conf>>>,
    context_and_param : &Conf::ContextAndParameterization,
    global_state : &Conf::MutablePersistentState
) {
    for logger in loggers_iter {
        logger.log_terminate_process(
            context_and_param,
            global_state
        );
    }
}