#[macro_use] extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            say_hello()
        }
    });

    server.listen("127.0.0.1:6767").unwrap();
}

fn say_hello() -> &'static str {
    "Hello world! (From function)"
}