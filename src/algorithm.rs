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

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

use map::Map;
use map::Node;
use map::Direction;
use map::Point;

#[derive(Eq, PartialEq)]
pub struct Path {
    pub dirs: String,
    pub cost: u32,
    last_node_id: usize,
}

struct NextPath {
    node_id: usize,
    dir: char,
}

macro_rules! start_path {
    ($x:expr) => {
        {
            Path {last_node_id: $x, cost: 0, dirs: String::new()}
        }
    };
}

macro_rules! next_path {
    ($map:expr, $node:expr, $dir:expr) => {
        {
            NextPath {dir: (*$dir as u8) as char, node_id: get_next_node_id(&$map, &$node, &$dir)}
        }
    };
}

impl Ord for Path {
    fn cmp(&self, other: &Path) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(map: &Map, start_point: Point, end_point: Point) -> Path {
    let start = map.get_node_id(&start_point);
    let end = map.get_node_id(&end_point);
    let mut heap: BinaryHeap<Path> = BinaryHeap::new();

    heap.push(start_path!(start));

    let mut visited_nodes: HashSet<usize> = HashSet::new();
    while let Some(path) = heap.pop() {
        let curr_pos = path.last_node_id;
        if curr_pos == end {
            // in destination
            return path;
        }

        let curr_node = map.get_node(curr_pos);
        let next_paths = get_next_paths(&map, &curr_node);
        for next_path in next_paths {
            if !visited_nodes.contains(&next_path.node_id) {
                heap.push(Path {
                    last_node_id: next_path.node_id,
                    cost: path.cost + map.get_node(next_path.node_id).value,
                    dirs: format!("{}{}", path.dirs, next_path.dir),
                });
                visited_nodes.insert(next_path.node_id);
            }
        }

        //println!("Heap[{}]: {:?}", heap.len(), heap);
    }
    return start_path!(start);
}

fn get_next_paths(map: &Map, node: &Node) -> Vec<NextPath> {
    let mut result = Vec::new();
    for direction in &node.paths {
        result.push(next_path!(map, node, direction));
    }
    return result;
}

fn get_next_node_id(map: &Map, last_node: &Node, dir: &Direction) -> usize {
    let next_point = move_to_dir(&last_node.pos, dir);
    return map.get_node_id(&next_point);
}

fn move_to_dir(from: &Point, dir: &Direction) -> Point {
    match *dir {
        Direction::RIGHT => {
            return Point {
                x: from.x,
                y: from.y + 1,
            }
        }
        Direction::LEFT => {
            return Point {
                x: from.x,
                y: from.y - 1,
            }
        }
        Direction::UP => {
            return Point {
                x: from.x - 1,
                y: from.y,
            }
        }
        Direction::DOWN => {
            return Point {
                x: from.x + 1,
                y: from.y,
            }
        }
    }
}