use std::io::prelude::*;
use std::fs::File;
use std::io;

fn main(){
     match log_something("log.txt", b"Its alive!!!"){
        Ok(..) => println!("file created"),
        Err(..) => println!("Error: could not create file")
     }


}

fn log_something(filename: &'static str, string: &'static [u8; 12]) -> io::Result<()> {
    let mut f = (File::create(filename))?;
    (f.write_all(string))?;
    Ok(())
}