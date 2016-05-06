extern crate hyper;
use hyper::server::*;

use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

struct RollingVector {
    max_size: i32,
    values: Vec<String>,
}

impl RollingVector {
    fn push(&mut self, item: String) {
        let l = self.values.len();
        if l > 2 {
            self.values.drain(0..l - 2);
        }
        self.values.push(item);
    }

    fn new(size: i32) -> RollingVector {
        RollingVector {
            max_size: size,
            values: Vec::new(),
        }
    }
}

fn main() {
    let data = Arc::new(Mutex::new(RollingVector::new(3)));
    let data2 = data.clone();
    let http_callback = move |_: Request, res: Response| {
        let d = data.clone();
        let ans = d.lock().unwrap().values.join("");
        res.send(&ans.into_bytes()[..]);
    };

    thread::spawn(move || Server::http("localhost:8080").unwrap().handle(http_callback));
    let mut f = File::open("/home/tomasz/log").unwrap();
    f.seek(SeekFrom::End(0));
    let mut reader = BufReader::new(f);
    loop {
        let mut s: String = String::new();
        let res = reader.read_line(&mut s);
        if let Ok(n) = res {
            if (n > 0) {
                data2.lock().unwrap().push(s);
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
