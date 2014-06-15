#![crate_id = "responsetime"]
#![comment = "Response time logging middleware for Iron"]
#![license = "MIT"]

//! Crate for the response time logging middleware

extern crate iron;
extern crate time;

use iron::request::Request;
use iron::response::Response;
use iron::ingot::{Status, Continue, Ingot};
use iron::alloy::Alloy;

use time::precise_time_ns;

#[deriving(Clone)]
pub struct ResponseTime {
    entry: u64
}

impl ResponseTime {
    pub fn new() -> ResponseTime {
        ResponseTime { entry: 0u64 }
    }
}

impl<'a, Rq: Request, Rs: Response<'a>> Ingot<'a, Rq, Rs> for ResponseTime {
    fn enter(&mut self, _request: &mut Rq, _response: &mut Rs, _alloy: &mut Alloy) -> Status {
        self.entry = precise_time_ns();
        Continue
    }
    fn exit(&mut self, _request: &mut Rq, _response: &mut Rs, _alloy: &mut Alloy) -> Status {
        let delta = precise_time_ns() - self.entry;
        println!("{} ms", (delta as f64) / 1000000f64);
        Continue
    }
}
