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

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    UP = 'U' as isize,
    DOWN = 'D' as isize,
    LEFT = 'L' as isize,
    RIGHT = 'R' as isize,
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

pub struct Node {
    pub value: u32,
    pub pos: Point,
    pub paths: Vec<Direction>,
}

pub struct Map {
    nodes: Vec<Node>,
    dim: usize,
}

macro_rules! point {
    ($x:expr, $y:expr) => {
        {
            Point {x: $x, y: $y}
        }
    };
}

macro_rules! node {
    ($value:expr, $paths:expr, $pos:expr) => {
        {
            Node { value: $value, paths: $paths, pos: $pos }
        }
    };
}

impl Map {
    pub fn new() -> Map {
        return Map {
            nodes: vec![],
            dim: 0,
        };
    }

    pub fn init(&mut self, nodes: &Vec<String>) {
        self.nodes = Vec::new();
        self.dim = (nodes.len() as f64).sqrt() as usize;
        for i in 0..nodes.len() {
            let pos = point!(i / self.dim, i % self.dim);
            let node = self.parse_node(&nodes[i], pos);
            self.nodes.push(node);
        }
    }

    pub fn get_rows(&self) -> Vec<&[Node]> {
        let mut result: Vec<&[Node]> = Vec::new();
        let rows = self.nodes.chunks(self.dim);
        for row in rows {
            result.push(row);
        }
        return result;
    }

    pub fn get_node(&self, id: usize) -> &Node {
        debug_assert!(id >= 0 && id < self.nodes.len());
        return self.nodes.get(id).unwrap();
    }

    pub fn get_node_id(&self, point: &Point) -> usize {
        debug_assert!(point.x >= 0 && point.x < self.dim);
        debug_assert!(point.y >= 0 && point.y < self.dim);
        return point.x * self.dim + point.y;
    }

    fn parse_node(&mut self, value: &String, point: Point) -> Node {
        debug_assert!(!value.is_empty());
        let vec: Vec<&str> = value.split("-").collect();
        debug_assert!(vec.len() == 2);
        let node_val = vec[0].parse::<u32>().unwrap();
        let node_paths = self.parse_paths(vec[1]);
        return node!(node_val, node_paths, point);
    }

    fn parse_paths(&mut self, paths: &str) -> Vec<Direction> {
        let mut result = Vec::new();
        for c in paths.chars() {
            match c {
                'R' => result.push(Direction::RIGHT),
                'L' => result.push(Direction::LEFT),
                'D' => result.push(Direction::DOWN),
                'U' => result.push(Direction::UP),
                _ => {}
            }
        }
        return result;
    }
}