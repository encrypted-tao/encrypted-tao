use crate::query::query::{Arg, AssocType, ObjType, Query, TaoArgs, TaoOp};
use pest::{self, Parser};
use std::str::FromStr;

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
        ("ASSOC", "DELETE") => TaoOp::AssocDelete,
        ("ASSOC", "CHTYPE") => TaoOp::AssocChangeType,
        ("ASSOC", "GET") => TaoOp::AssocGet,
        ("ASSOC", "RGET") => TaoOp::AssocRangeGet,
        ("ASSOC", "COUNT") => TaoOp::AssocCount,
        ("ASSOC", "RANGE") => TaoOp::AssocRange,
        ("ASSOC", "TRANGE") => TaoOp::AssocTimeRange,
        ("OBJ", "ADD") => TaoOp::ObjAdd,
        ("OBJ", "GET") => TaoOp::ObjGet,
        ("OBJ", "DELETE") => TaoOp::ObjDelete,
        ("OBJ", "UPDATE") => TaoOp::ObjUpdate,
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
            let id1 = Arg::UID(a1.parse().unwrap());
            let atype = Arg::AssocType(AssocType::from_str(a2).unwrap());
            let id2 = Arg::UID(a3.parse().unwrap());
            let time = Arg::Num(a4.parse().unwrap());
            let data = Arg::Str(a5.to_string());

            return TaoArgs::FiveArgs {
                arg1: id1,
                arg2: atype,
                arg3: id2,
                arg4: time,
                arg5: data,
            };
        }
        TaoOp::AssocDelete => {
            let (a1, a2, a3) = unwrap_three_args(args);
            let id1 = Arg::UID(a1.parse().unwrap());
            let atype = Arg::AssocType(AssocType::from_str(a2).unwrap());
            let id2 = Arg::UID(a3.parse().unwrap());

            return TaoArgs::ThreeArgs {
                arg1: id1,
                arg2: atype,
                arg3: id2,
            };
        }
        TaoOp::AssocChangeType => {
            let (a1, a2, a3, a4) = unwrap_four_args(args);
            let id1 = Arg::UID(a1.parse().unwrap());
            let atype = Arg::AssocType(AssocType::from_str(a2).unwrap());
            let id2 = Arg::UID(a3.parse().unwrap());
            let natype = Arg::AssocType(AssocType::from_str(a4).unwrap());

            return TaoArgs::FourArgs {
                arg1: id1,
                arg2: atype,
                arg3: id2,
                arg4: natype,
            };
        }
        TaoOp::AssocGet => {
            let (a1, a2, a3) = unwrap_three_args(args);
            let id1 = Arg::UID(a1.parse().unwrap());
            let atype = Arg::AssocType(AssocType::from_str(a2).unwrap());
            let idset = Arg::UIDSet(parse_id_set(a3));

            return TaoArgs::ThreeArgs {
                arg1: id1,
                arg2: atype,
                arg3: idset,
            };
        }
        TaoOp::AssocRangeGet => {
            let (a1, a2, a3, a4, a5) = unwrap_five_args(args);
            let id1 = Arg::UID(a1.parse().unwrap());
            let atype = Arg::AssocType(AssocType::from_str(a2).unwrap());
            let idset = Arg::UIDSet(parse_id_set(a3));
            let t1 = Arg::Num(a4.parse().unwrap());
            let t2 = Arg::Num(a5.parse().unwrap());

            return TaoArgs::FiveArgs {
                arg1: id1,
                arg2: atype,
                arg3: idset,
                arg4: t1,
                arg5: t2,
            };
        }
        TaoOp::AssocCount => {
            let (a1, a2) = unwrap_two_args(args);
            let id = Arg::UID(a1.parse().unwrap());
            let atype = Arg::AssocType(AssocType::from_str(a2).unwrap());

            return TaoArgs::TwoArgs {
                arg1: id,
                arg2: atype,
            };
        }
        TaoOp::AssocRange => {
            let (a1, a2, a3, a4) = unwrap_four_args(args);
            let id1 = Arg::UID(a1.parse().unwrap());
            let atype = Arg::AssocType(AssocType::from_str(a2).unwrap());
            let r1 = Arg::Num(a3.parse().unwrap());
            let r2 = Arg::Num(a4.parse().unwrap());

            return TaoArgs::FourArgs {
                arg1: id1,
                arg2: atype,
                arg3: r1,
                arg4: r2,
            };
        }
        TaoOp::AssocTimeRange => {
            let (a1, a2, a3, a4, a5) = unwrap_five_args(args);
            let id1 = Arg::UID(a1.parse().unwrap());
            let atype = Arg::AssocType(AssocType::from_str(a2).unwrap());
            let t1 = Arg::Num(a3.parse().unwrap());
            let t2 = Arg::Num(a4.parse().unwrap());
            let lim = Arg::Num(a5.parse().unwrap());

            return TaoArgs::FiveArgs {
                arg1: id1,
                arg2: atype,
                arg3: t1,
                arg4: t2,
                arg5: lim,
            };
        }
        TaoOp::ObjAdd => {
            let (a1, a2, a3) = unwrap_three_args(args);
            let id: i32 = a1.parse().unwrap();
            let otype = Arg::ObjType(ObjType::from_str(a2).unwrap());
            println!("{:#?}\n", a3);
            let data = a3.to_string();

            return TaoArgs::ObjAddArgs {
                arg1: id,
                arg2: otype.to_string(),
                arg3: data,
            };
        }
        TaoOp::ObjGet => {
            // let mut args = args.into_inner();
            let id: i32 = args.next().unwrap().as_str().parse().unwrap();

            return TaoArgs::ObjGetArgs { arg1: id };
        }
        TaoOp::ObjDelete => {
            // let mut args = args.into_inner();
            let id = Arg::UID(args.next().unwrap().as_str().parse().unwrap());

            return TaoArgs::OneArgs { arg1: id };
        }
        _ => panic!("Invalid Query Arguments"),
    };
}

fn parse_id_set(lst: &str) -> Vec<Arg> {
    let mut ids = TaoParser::parse(Rule::NumList, lst)
        .unwrap_or_else(|e| panic!("{}", e));
    let ids = ids.next().unwrap();
    let ids = ids.into_inner().filter(|p| match p.as_rule() {
        Rule::Number => true,
        _ => panic!("Set of IDs should only contain numbers"),
    });
    let idset = ids.map(|n| Arg::UID(n.as_str().parse().unwrap()));
    let idset = idset.collect::<Vec<Arg>>();

    return idset;
}

// Embarrassing!
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

fn unwrap_four_args(
    mut args: pest::iterators::Pairs<Rule>,
) -> (&str, &str, &str, &str) {
    // let mut args = args.into_inner();
    let a1 = args.next().unwrap().as_str();
    let a2 = args.next().unwrap().as_str();
    let a3 = args.next().unwrap().as_str();
    let a4 = args.next().unwrap().as_str();

    return (a1, a2, a3, a4);
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
