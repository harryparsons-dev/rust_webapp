extern crate chrono;

use chrono::{DateTime, Local};
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::io;

fn main(){


     match log_time("log.txt"){
        Ok(..) => println!("file created"),
        Err(e) => println!("Error: {}", e)
     }


}

fn log_time(filename: &'static str) -> io::Result<()> {
    let entry = formatted_time_entry();
    let bytes = entry.as_bytes();

    record_entry_log(filename, &bytes)?;
    Ok(())

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