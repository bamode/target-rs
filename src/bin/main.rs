use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf: [u8; 2880] = [0; 2880];
    /* 
    for _ in 0..6 {
        let len = reader.read(&mut buf).unwrap();
        let line: String = buf.iter().map(|b: &u8| -> char {*b as char}).collect::<String>();
        println!("{:?}", &line[0..len].split("=").collect::<Vec<&str>>().iter().map(|s| s.trim()).collect::<Vec<&str>>());
    }
    
    loop {
        let len = reader.read(&mut buf).unwrap();
        println!("{:?}", &buf[0..len]);
        if len == 0 { break; }
    }
    */
    for _ in 0..2 {
        let len = reader.read(&mut buf).unwrap();
        let line: String = buf.iter().map(|b: &u8| -> char {*b as char}).collect::<String>();
        let res = parse_header(&line[0..len]);
        println!("{:?}", res);
    }
    
}

pub fn parse_keyword(s: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::take_until(" ")(s)
}

pub fn parse_header(s: &str) -> Vec<&str> {
    let mut res: Vec<&str> = Vec::new();
    for _ in 0..5 {
        let (rest, keyword): (&str, &str) = parse_keyword(s).unwrap();
        let rest = rest.trim();
        res.push(keyword);
        res.push(rest);
    }

    res
}