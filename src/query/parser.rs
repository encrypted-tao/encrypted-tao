// assoc_add(id1, atype, id2, time, (k->v)*)        5
use pest::{self, Parser};
// use crate::query::ast::{TaoOp, ObjType, AssocType, ArgType, Query};

#[derive(pest_derive::Parser)]
#[grammar = "query/tao.pest"]
struct TaoParser;

pub fn parse(source: &str) {
    let pairs = TaoParser::parse(Rule::Program, source).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        println!("Text: {}", pair.as_str());
    }
}
