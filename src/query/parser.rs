use pest::{self, Parser};

use crate::query::query::{Query, TaoArgs, TaoOp};

#[derive(pest_derive::Parser)]
#[grammar = "query/tao.pest"]
struct TaoParser;

pub fn parse(source: &str) -> Vec<Query> {
    let mut program = TaoParser::parse(Rule::Queries, source)
        .unwrap_or_else(|e| panic!("{}", e));
    let prog = program.next().unwrap();
    let qs = match prog.as_rule() {
        Rule::Queries => parse_queries(prog),
        _ => panic!("Invalid Query Input!"),
    };

    return qs;
}

fn parse_queries(queries: pest::iterators::Pair<Rule>) -> Vec<Query> {
    let qs = queries.into_inner().filter(|p| match p.as_rule() {
        Rule::Query => true,
        _ => false,
    });
    return qs.map(|q| parse_query(q)).collect::<Vec<Query>>();
}

fn parse_query(query: pest::iterators::Pair<Rule>) -> Query {
    let mut query_body = query.into_inner().next().unwrap().into_inner();
    let target = query_body.next().unwrap();
    let op = query_body.next().unwrap();

    let tao_op = parse_tao_op(target.as_str(), op.as_str());
    let tao_args = parse_tao_args(&tao_op, query_body);

    return Query {
        op: tao_op,
        args: tao_args,
    };
}

fn parse_tao_op(target: &str, op: &str) -> TaoOp {
    let tao_op = match (target, op) {
        ("ASSOC", "ADD") => TaoOp::AssocAdd,
        ("ASSOC", "GET") => TaoOp::AssocGet,
        ("ASSOC", "RGET") => TaoOp::AssocRangeGet,
        ("ASSOC", "COUNT") => TaoOp::AssocCount,
        ("ASSOC", "RANGE") => TaoOp::AssocRange,
        ("OBJ", "ADD") => TaoOp::ObjAdd,
        ("OBJ", "GET") => TaoOp::ObjGet,
        _ => panic!("Invalid target and operation combination"),
    };
    return tao_op;
}

fn parse_tao_args(
    op: &TaoOp,
    mut args: pest::iterators::Pairs<Rule>,
) -> TaoArgs {
    match op {
        TaoOp::AssocAdd => {
            let (a1, a2, a3, a4, a5) = unwrap_five_args(args);
            let id1: String = a1.to_string();
            let atype: String = a2.to_string();
            let id2: String = a3.to_string();
            let time: i64 = a4.parse().unwrap();
            let data: String = a5.to_string();

            return TaoArgs::AssocAddArgs {
                id1: id1,
                atype: atype,
                id2: id2,
                time: time,
                data: data,
            };
        }
        TaoOp::AssocDelete => panic!("Operation not supported"),
        TaoOp::AssocChangeType => panic!("Operation not supported"),
        TaoOp::AssocGet => {
            let (a1, a2, a3) = unwrap_three_args(args);
            let id: String = a1.to_string();
            let atype: String = a2.to_string();
            let idset: Vec<String> = parse_id_set(a3);

            return TaoArgs::AssocGetArgs {
                id: id,
                atype: atype,
                idset: idset,
            };
        }
        TaoOp::AssocRangeGet => {
            let (a1, a2, a3, a4, a5) = unwrap_five_args(args);
            let id: String = a1.to_string();
            let atype: String = a2.to_string();
            let idset: Vec<String> = parse_id_set(a3);
            let tstart: i64 = a4.parse().unwrap();
            let tend: i64 = a5.parse().unwrap();

            return TaoArgs::AssocRangeGetArgs {
                id: id,
                atype: atype,
                idset: idset,
                tstart: tstart,
                tend: tend,
            };
        }
        TaoOp::AssocCount => {
            let (a1, a2) = unwrap_two_args(args);
            let id: String = a1.to_string();
            let atype: String = a2.to_string();

            return TaoArgs::AssocCountArgs {
                id: id,
                atype: atype,
            };
        }
        TaoOp::AssocRange => {
            let (a1, a2, a3, a4, a5) = unwrap_five_args(args);
            let id1: String = a1.to_string();
            let atype = a2.to_string();
            let t1: i64 = a3.parse().unwrap();
            let t2: i64 = a4.parse().unwrap();
            let lim: i64 = a5.parse().unwrap();

            return TaoArgs::AssocRangeArgs {
                id: id1,
                atype: atype,
                tstart: t1,
                tend: t2,
                lim: lim,
            };
        }
        TaoOp::ObjAdd => {
            let (a1, a2, a3) = unwrap_three_args(args);
            let id: String = a1.to_string();
            let otype = a2.to_string();
            let data = a3.to_string();

            return TaoArgs::ObjAddArgs {
                id: id,
                otype: otype,
                data: data,
            };
        }
        TaoOp::ObjGet => {
            let id: String = args.next().unwrap().as_str().to_string();

            return TaoArgs::ObjGetArgs { id: id };
        }
        TaoOp::ObjDelete => panic!("Operation not supported"),
        _ => panic!("Operation not supported"),
    };
}

fn parse_id_set(lst: &str) -> Vec<String> {
    let mut ids = TaoParser::parse(Rule::NumList, lst)
        .unwrap_or_else(|e| panic!("{}", e));
    let ids = ids.next().unwrap();
    let ids = ids.into_inner().filter(|p| match p.as_rule() {
        Rule::Number => true,
        _ => panic!("Set of IDs should only contain numbers"),
    });
    let idset = ids.map(|n| n.as_str().parse().unwrap());
    let idset = idset.collect::<Vec<String>>();

    return idset;
}

// not really clean
fn unwrap_two_args(mut args: pest::iterators::Pairs<Rule>) -> (&str, &str) {
    // let mut args = args.into_inner();
    let a1 = args.next().unwrap().as_str();
    let a2 = args.next().unwrap().as_str();
    return (a1, a2);
}

fn unwrap_three_args(
    mut args: pest::iterators::Pairs<Rule>,
) -> (&str, &str, &str) {
    // let mut args = args.into_inner();
    let a1 = args.next().unwrap().as_str();
    let a2 = args.next().unwrap().as_str();
    let a3t = args.next().unwrap();
    let a3 = match a3t.as_rule() {
        Rule::String => a3t.into_inner().next().unwrap().as_str(),
        _ => a3t.as_str(),
    };
    return (a1, a2, a3);
}

fn unwrap_five_args(
    mut args: pest::iterators::Pairs<Rule>,
) -> (&str, &str, &str, &str, &str) {
    // let mut args = args.into_inner();
    let a1 = args.next().unwrap().as_str();
    let a2 = args.next().unwrap().as_str();
    let a3 = args.next().unwrap().as_str();
    let a4 = args.next().unwrap().as_str();
    let a5t = args.next().unwrap();

    let a5 = match a5t.as_rule() {
        Rule::String => a5t.into_inner().next().unwrap().as_str(),
        _ => a5t.as_str(),
    };

    return (a1, a2, a3, a4, a5);
}
