extern crate iron;
extern crate responsetime;

use std::io::net::ip::Ipv4Addr;

use iron::iron::Iron;
use iron::furnace::ironfurnace::IronFurnace;
use iron::request::ironrequest::IronRequest;
use iron::response::ironresponse::IronResponse;

use responsetime::ResponseTime;

fn main() {
    let mut server: Iron<IronRequest, IronResponse<'static>, IronFurnace<IronRequest, IronResponse<'static>>> =
        Iron::new();
    server.smelt(ResponseTime::new());
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
