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
 
extern crate serde_json;
extern crate curl;

use std::fs::File;
use std::io::Read;
use std::str;

use self::curl::easy::Easy;
use self::curl::easy::List;

use map::Point;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct TaskRespMap {
     pub areas: Vec<String>
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct TaskResp {
    pub id: String,
    pub startedTimestamp: u64,
    pub map: TaskRespMap,
    pub astroants: Point,
    pub sugar: Point
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct TaskResultReq {
    path: String
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct TaskResultResp {
    valid: bool,
    inTime: bool
}

pub fn post_result(id: &String, dirs: String) -> bool {
    let result_req = TaskResultReq { path: dirs };
    let json = serde_json::to_string(&result_req).unwrap();
    let url = format!("http://tasks-rad.quadient.com:8080/task/{}", id);
    let mut response = Vec::new();
    put_json_to_url(str::from_utf8(url.as_bytes()).unwrap(), &json, &mut response);
    let json = String::from_utf8(response).unwrap();
    let result_resp: TaskResultResp = serde_json::from_str(&json).unwrap();
    return result_resp.valid && result_resp.inTime;
}

pub fn load_from_url() -> TaskResp {
    let mut response = Vec::new();
    get_json_from_url("http://tasks-rad.quadient.com:8080/task", &mut response);
    let json = String::from_utf8(response).unwrap();
    return serde_json::from_str(&json).unwrap();
}

pub fn load_from_file(filePath: &str) -> TaskResp {
    let mut f = File::open(filePath).expect("file not found");
    let mut json = String::new();
    f.read_to_string(&mut json);
    return serde_json::from_str(&json).unwrap();
}

pub fn load_from_str(json: &str) -> TaskResp {
    return serde_json::from_str(&json).unwrap();
}

fn get_json_from_url(url: &str, response: &mut Vec<u8>) {
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            response.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
}

fn put_json_to_url(url: &str, json: &str, resp_data: &mut Vec<u8>) {
    let mut data = json.as_bytes();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    handle.put(true).unwrap();
    let mut headers = List::new();
    headers.append("Content-Type: application/json").unwrap();
    handle.http_headers(headers).unwrap();
    handle.post_field_size(data.len() as u64).unwrap();

    let mut transfer = handle.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.write_function(|new_data| {
        resp_data.extend_from_slice(new_data);
        Ok(new_data.len())
    }).unwrap();
    transfer.perform().unwrap();
}