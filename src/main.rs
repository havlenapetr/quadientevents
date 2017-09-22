/*
 * Copyright (C) 2017 Petr Havlena (havlenapetr@gmail.com)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[macro_use]
extern crate serde_derive;

use std::time::Instant;

mod algorithm;
mod task;
mod map;
mod utils;

use map::Map;

fn main() {
    println!("loading task from server ...");
    let task = task::load_from_url();
    
    let mut map = Map::new();
    map.init(&task.map.areas);
    println!("task {} parsed, solving ...", task.id);

    let now = Instant::now();
    let path = algorithm::run(&map, utils::mirrored(task.astroants), utils::mirrored(task.sugar));
    let elapsed_time = utils::get_time_in_us(now.elapsed());
    println!("task={} solved after {}us, costs={} and paths=[{}]", task.id, elapsed_time, path.cost, path.dirs);

    println!("checking result ...");
    assert!(task::post_result(&task.id, path.dirs));
    println!("result is valid and in time");
}