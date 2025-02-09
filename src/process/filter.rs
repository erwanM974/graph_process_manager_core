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
 * Filter than can be applied upon reaching a new node in the graph,
 * before computing the steps that can be fired from it.
 * **/
pub trait AbstractNodePreFilter<Conf : AbstractProcessConfiguration> {

    fn apply_filter(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        global_state : &Conf::MutablePersistentState,
        node : &Conf::DomainSpecificNode
    ) -> Option<Conf::FiltrationResult>;

    /** 
     * Returns a title describing the filter.
     * **/
    fn get_filter_description(&self) -> &str;

}



/** 
 * Filter than can be applied upon reaching a new node in the graph,
 * after computing the steps that can be fired from it.
 * **/
 pub trait AbstractNodePostFilter<Conf : AbstractProcessConfiguration> {

    fn apply_filter(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        global_state : &Conf::MutablePersistentState,
        node : &Conf::DomainSpecificNode,
        next_steps : &[Conf::DomainSpecificStep]
    ) -> Option<Conf::FiltrationResult>;

    /** 
     * Returns a title describing the filter.
     * **/
    fn get_filter_description(&self) -> &str;

}




/** 
 * Filter than can be applied on the evaluation of a specific step.
 * **/
 pub trait AbstractStepFilter<Conf : AbstractProcessConfiguration> {

    fn apply_filter(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        global_state : &Conf::MutablePersistentState,
        parent_node : &Conf::DomainSpecificNode,
        step : &Conf::DomainSpecificStep
    ) -> Option<Conf::FiltrationResult>;

    /** 
     * Returns a title describing the filter.
     * **/
    fn get_filter_description(&self) -> &str;

}





pub struct GenericFiltersManager<Conf : AbstractProcessConfiguration> {
    node_pre_filters : Vec<Box<dyn AbstractNodePreFilter<Conf>>>,
    node_post_filters : Vec<Box<dyn AbstractNodePostFilter<Conf>>>,
    step_filters : Vec<Box<dyn AbstractStepFilter<Conf>>>
}


impl<Conf : 'static + AbstractProcessConfiguration> GenericFiltersManager<Conf> {

    pub fn new(
        node_pre_filters : Vec<Box<dyn AbstractNodePreFilter<Conf>>>,
        node_post_filters : Vec<Box<dyn AbstractNodePostFilter<Conf>>>,
        step_filters : Vec<Box<dyn AbstractStepFilter<Conf>>>
    ) -> Self {
        Self { node_pre_filters, node_post_filters, step_filters }
    }

    pub fn get_node_pre_filters(&self) -> &Vec<Box<dyn AbstractNodePreFilter<Conf>>> {
        &self.node_pre_filters
    }

    pub fn get_node_post_filters(&self) -> &Vec<Box<dyn AbstractNodePostFilter<Conf>>> {
        &self.node_post_filters
    }

    pub fn get_step_filters(&self) -> &Vec<Box<dyn AbstractStepFilter<Conf>>> {
        &self.step_filters
    }

    pub fn apply_node_pre_filters(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        global_state : &Conf::MutablePersistentState,
        node : &Conf::DomainSpecificNode
    ) -> Option<Conf::FiltrationResult> {
        for filter in &self.node_pre_filters {
            match filter.apply_filter(context_and_param,global_state,node) {
                None => {},
                Some( res) => {
                    return Some(res);
                }
            }
        }
        None
    }

    pub fn apply_node_post_filters(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        global_state : &Conf::MutablePersistentState,
        node : &Conf::DomainSpecificNode,
        next_steps : &[Conf::DomainSpecificStep]
    ) -> Option<Conf::FiltrationResult> {
        for filter in &self.node_post_filters {
            match filter.apply_filter(context_and_param,global_state,node,next_steps) {
                None => {},
                Some( res) => {
                    return Some(res);
                }
            }
        }
        None
    }

    pub fn apply_step_filters(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        global_state : &Conf::MutablePersistentState,
        parent_node : &Conf::DomainSpecificNode,
        step : &Conf::DomainSpecificStep
    ) -> Option<Conf::FiltrationResult> {
        for filter in &self.step_filters {
            match filter.apply_filter(context_and_param,global_state,parent_node,step) {
                None => {},
                Some( res) => {
                    return Some(res);
                }
            }
        }
        None
    }

}


