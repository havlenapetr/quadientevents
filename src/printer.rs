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
 
use map::Map;
use map::Node;
use map::Directon;

pub fn print_map(map: &Map) {
    for row in map.get_rows() {
        for x in 0..row.len() {
            print!("  ");
            if !print_direction(&row[x], Directon::UP) {
                print!("   ");
            } else {
                print!("  ");
            }
        }

        println!();
        for x in 0..row.len() {
            print_node_horizontaly(&row[x]);
        }
        println!();

        for x in 0..row.len() {
            print!("  ");
            if !print_direction(&row[x], Directon::DOWN) {
                print!("   ");
            } else {
                print!("  ");
            }
        }

        println!();
    }
}

fn print_node_horizontaly(node: &Node) {
    if !print_direction(&node, Directon::LEFT) {
        print!(" ");
    }
    print!("({})", node.value);
    if !print_direction(&node, Directon::RIGHT) {
        print!(" ");
    }
}

fn print_direction(node: &Node, dir: Directon) -> bool {
    if node.paths.contains(&dir) {
        match dir {
            Directon::RIGHT => print!("-"),
            Directon::LEFT => print!("-"),
            Directon::UP => print!("|"),
            Directon::DOWN => print!("|")
        }
        return true;
    }
    return false;
}