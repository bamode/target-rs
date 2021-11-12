extern crate nom;
use nom::{IResult, bytes::complete::{take_until}};

fn parse_keyword(s: &str) -> IResult<&str, &str> {
    take_until(" ")(s)
}

fn parse_comment(s: &str) -> IResult<&str, &str> {
    unimplemented!()
}

pub fn parse_header(s: &str) -> Vec<&str> {
    let mut res: Vec<&str> = Vec::new();
    loop {
        let (rest, keyword): (&str, &str) = parse_keyword(s).unwrap();
        let rest = rest.trim();
        res.push(keyword);
        res.push(rest);
    }

    res
}