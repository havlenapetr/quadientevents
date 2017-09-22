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
 
include!("../src/main.rs");

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use map::Map;
    use map::Point;

    use task;
    use algorithm;
    use utils;

    #[test]
    fn get_rows_count() {
        let task_file: &'static str = include_str!("task3x3.txt");
        let task = task::load_from_str(task_file);

        let mut map = Map::new();
        map.init(&task.map.areas);

        let rows = map.get_rows();
        assert_eq!(3, rows.len());
    }

    #[test]
    fn get_node_returns_right_node_for_position() {
        let paths = vec![
            String::from("5-R"),
            String::from("1-RDL"),
            String::from("10-DL"),
            String::from("2-RD"),
            String::from("1-UL"),
            String::from("1-UD"),
            String::from("2-RU"),
            String::from("1-RL"),
            String::from("2-UL"),
        ];
        let mut map = Map::new();
        map.init(&paths);

        let point = Point { x: 1, y: 1 };
        let node_id = map.get_node_id(&point);
        let node = map.get_node(node_id);
        assert_eq!(node.pos.x, point.x);
        assert_eq!(node.pos.y, point.y);
    }

    #[test]
    fn algorithm_result() {
        let task_file: &'static str = include_str!("task3x3.txt");
        let task = task::load_from_str(task_file);

        let mut map = Map::new();
        map.init(&task.map.areas);

        let path = algorithm::run(&map, utils::mirrored(task.astroants), utils::mirrored(task.sugar));
        assert_eq!("DLDRRU", path.dirs);
    }

    #[test]
    fn algorithm_speed_must_be_under_5_seconds() {
        let task_file: &'static str = include_str!("task300x300.txt");
        let task = task::load_from_str(task_file);

        let mut map = Map::new();
        map.init(&task.map.areas);

        let now = Instant::now();
        algorithm::run(&map, utils::mirrored(task.astroants), utils::mirrored(task.sugar));
        let elapsed_time = utils::get_time_in_us(now.elapsed());

        assert!(elapsed_time < 5_000_000);
    }

    #[test]
    fn algorithm_test_against_url() {
        let task = task::load_from_url();

        let mut map = Map::new();
        map.init(&task.map.areas);

        let path = algorithm::run(&map, utils::mirrored(task.astroants), utils::mirrored(task.sugar));
        assert!(task::post_result(&task.id, path.dirs));
    }
}