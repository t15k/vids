extern crate clap;

use std::io::Write;
use std::fs::File;
use std::io::Read;
use clap::{App,Arg};

fn main() {
    let matches = App::new("vids")
        .version("0.1.0")
        .about("streams input to a var in delimeted stream. ")
        .arg(Arg::with_name("files")
            .index(1)
            .multiple(true))
        .get_matches();
    match matches.values_of("files") {
        Some(files) => for filename in files {
            let mut buf = [0; 2048];
            let mut file = File::open(filename).expect("could not open file");
            let md = file.metadata().expect("could not read metadata for file");
            let size = md.len();//.expect("could not read size");
            write_varint(size, &mut std::io::stdout());
            loop {
                match file.read(&mut buf) {
                    Ok(0) => break,
                    Ok(read_count) => {
                        std::io::stdout().write(&buf[..read_count]).expect("could not write");
                    },
                    Err(e) => panic!(e)
                }
                
            }
            // 
            println!("{}", filename);
        },
        None => ()
    }
}

fn write_varint(n :u64, w :&mut Write) {
    let mut a = [0; 5];
    let mut v = n;
    let mut i = 0;
    loop {
        a[i] = (v & 0b0111_1111) as u8;
        v = v >> 7;
        if v > 0 { a[i] = a[i] | 0b1000_0000; }
        i += 1;
        if v == 0 { break; }
    }
    w.write(&a[..i]).expect("could not write");
}
