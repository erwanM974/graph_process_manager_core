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
use crate::queued_steps::step::GenericStep;

pub trait AbstractProcessHandler<Context,Step,Node,Criterion> {

    fn process_new_step(context : &Context,
                        parent_state : &GenericNode<Node>,
                        step_to_process : &GenericStep<Step>,
                        new_state_id : u32,
                        node_counter : u32) -> Node;

    fn get_criterion(context : &Context,
                     parent_state : &GenericNode<Node>,
                     step_to_process : &GenericStep<Step>,
                     new_state_id : u32,
                     node_counter : u32) -> Criterion;

    fn collect_next_steps(context : &Context,
                          parent_state_id : u32,
                          parent_node_kind : &Node) -> (u32,Vec<GenericStep<Step>>);

}

