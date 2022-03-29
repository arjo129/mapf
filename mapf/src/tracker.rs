/*
 * Copyright (C) 2022 Open Source Robotics Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/

pub trait Tracker<Node>: Default {
    fn expanded_to(&mut self, node: &Node);
    fn solution_found_from(&mut self, node: &Node);
}


#[derive(Default)]
pub struct NoDebug;

impl<Node> Tracker<Node> for NoDebug {
    fn expanded_to(&mut self, _: &Node) { }
    fn solution_found_from(&mut self, _: &Node) { }
}