#[macro_use] extern crate nickel;
extern crate chrono;

use nickel::Nickel;
use chrono::{DateTime, Local};
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::io;
use clap::{Arg,App};

fn main() {
    // let matches = App::new("simple-log").version("v0.0.1")
    //     .arg(Arg::with_name("LOG FILE")
    //          .short("l")
    //          .long("logfile")
    //          .required(true)
    //          .takes_value(true))
    //     .arg(Arg::with_name("AUTH TOKEN")
    //         .short("t")
    //         .long("token")
    //         .takes_value(true))
    //     .get_matches();

    //     let logfile_path = matches.value_of("LOG FILE").unwrap();
    //     let auth_token = match matches.value_of("AUTH TOKEN") {
    //         Some(str) => Some(str.to_string()),
    //         None => None
    //     };

        let mut server = Nickel::new();

        server.utilize(router! {
            get "**" => |_req, _res| {
                do_log_time()
            }
        });

        server.listen("127.0.0.1:6767").unwrap();



}



fn do_log_time() -> String{


     match log_time("log.txt"){
        Ok(entry) => format!("Entry Logged: {}", entry),
        Err(e) => format!("Error: {}", e)
     }


}

fn log_time(filename: &'static str) -> io::Result<String> {
    let entry = formatted_time_entry();
    {
        let bytes = entry.as_bytes();

        record_entry_log(filename, &bytes)?;
    }
    Ok(entry)

}

fn formatted_time_entry() -> String{
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn record_entry_log(filename: &str, bytes: &[u8]) -> io::Result<()>{
    let mut file = (OpenOptions::new().
                    append(true).
                    write(true).
                    create(true).
                    open(filename))?;
    (file.write_all(bytes))?;
    Ok(())
}


