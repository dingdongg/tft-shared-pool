use serde::{Serialize, Deserialize};
use std::{
    fs,
    collections::HashMap, 
    net::{
        TcpListener,
        TcpStream,
    }, 
    io::{
        prelude::*,
        BufReader,
    },
    thread,
    time::Duration,
};
use pool::UnitsStore;
mod util;
mod pool;

#[derive(Debug, Serialize, Deserialize)]
pub struct BetterHTTPResponse {
    sets: HashMap<u32, BetterSetResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BetterSetResponse {
    champions: Vec<BetterSetChampionsResponse>,
    name: String,
    traits: Vec<BetterSetTraitsResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BetterSetChampionsResponse {
    traits: Vec<String>,
    cost: u32,
    name: String,
    apiName: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BetterSetTraitsResponse {
    apiName: String,
    desc: String,
    icon: String,
    name: String,
}

fn main() {
    let store = UnitsStore::new();
    store.check_pool();

    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &req_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        },
        _ => ("HTTP/1.1 404 Not Found", "error.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    println!("res: {status_line}");
    stream.write_all(response.as_bytes()).unwrap();
}