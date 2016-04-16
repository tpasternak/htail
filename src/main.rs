extern crate hyper;

use hyper::server::*;

use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{BufRead, BufReader};

fn hello(_:Request, res:Response){
    res.send(b"Hello world").unwrap();
}

fn main() {

//    Server::http("localhost:8080").unwrap().handle(hello).unwrap();

    let mut f = File::open("/var/log/syslog").unwrap();
    f.seek(SeekFrom::End(0));
    let mut b = BufReader::new(f);
    loop{
        let mut s: String = String::new();
        let res = b.read_line(&mut s);
        match res {
            Ok(0) => {},
            Ok(_) => print!("{}", s),
            Err(_) => {},
        }

    }

}
