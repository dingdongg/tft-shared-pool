use serde::{Serialize, Deserialize};
use bruh::ThreadPool;
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
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};
use pool::UnitsStore;
use lazy_static::lazy_static;

use util::{make_unit, TFTTrait};
mod util;
mod pool;
mod bruh;

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
    let store = Arc::new(Mutex::new(UnitsStore::new()));

    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let t_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        // see https://users.rust-lang.org/t/defining-a-global-mutable-structure-to-be-used-across-several-threads/7872/4
        let lol = Arc::clone(&store);
        let stream = stream.unwrap();

        t_pool.execute(move || {
            handle_connection(stream, lol);
        });
    }
}

fn handle_connection(mut stream: TcpStream, store: Arc<Mutex<UnitsStore>>) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    println!("{req_line}");

    let (status_line, filename) = match &req_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(3));
            ("HTTP/1.1 200 OK", "index.html")
        },
        "POST /buy HTTP/1.1" => {
            store.lock().unwrap().buy(&make_unit(
                "Jinx", 
                5, 
                vec![TFTTrait::Mixmaster, TFTTrait::Spellweaver], 
                false,
            ));
            ("HTTP/1.1 204 No Content", "buy.html")
        },
        "OPTIONS /buy HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        _ => ("HTTP/1.1 404 Not Found", "error.html"),
    };

    println!("filename::{filename}");

    if filename == "buy.html" {
        let response = format!("{status_line}\r\nAccess-Control-Allow-Origin: *\r\n");
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{contents}");
    // println!("res: {status_line}");
    stream.write_all(response.as_bytes()).unwrap();
}