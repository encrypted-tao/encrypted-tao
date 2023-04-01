// assoc_add(id1, atype, id2, time, (k->v)*)        5
use crate::query::ast::{Arg, AssocType, ObjType, Query, TaoArgs, TaoOp};
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
    let mut query_components = query.into_inner();
    let target = query_components.next().unwrap();
    let op = query_components.next().unwrap();
    let args = query_components
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap();

    let tao_op = parse_tao_op(target.as_str(), op.as_str());
    let tao_args = parse_tao_args(args);

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

fn parse_tao_args(args: pest::iterators::Pair<Rule>) -> TaoArgs {
    match args.as_rule() {
        Rule::AssocAddArgs => {
            let (a1, a2, a3, a4, a5, a6) = unwrap_six_args(args);
            let id1 = Arg::UID(a1.parse().unwrap());
            let atype = Arg::AssocType(AssocType::from_str(a2).unwrap());
            let id2 = Arg::UID(a3.parse().unwrap());
            let time = Arg::Num(a4.parse().unwrap());
            let k = Arg::Str(a5.to_string());
            let v = Arg::Str(a6.to_string());

            return TaoArgs::SixArgs {
                arg1: id1,
                arg2: atype,
                arg3: id2,
                arg4: time,
                arg5: k,
                arg6: v,
            };
        }
        Rule::AssocDeleteArgs => {
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
        Rule::AssocChTypeArgs => {
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
        Rule::AssocGetArgs => {
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
        Rule::AssocRGetArgs => {
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
        Rule::AssocCountArgs => {
            let (a1, a2) = unwrap_two_args(args);
            let id = Arg::UID(a1.parse().unwrap());
            let atype = Arg::AssocType(AssocType::from_str(a2).unwrap());

            return TaoArgs::TwoArgs {
                arg1: id,
                arg2: atype,
            };
        }
        Rule::AssocRangeArgs => {
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
        Rule::AssocTRangeArgs => {
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
        Rule::ObjAddArgs => {
            let (a1, a2, a3, a4) = unwrap_four_args(args);
            let id = Arg::UID(a1.parse().unwrap());
            let otype = Arg::ObjType(ObjType::from_str(a2).unwrap());
            let k = Arg::Str(a3.to_string());
            let v = Arg::Str(a4.to_string());

            return TaoArgs::FourArgs {
                arg1: id,
                arg2: otype,
                arg3: k,
                arg4: v,
            };
        }
        Rule::ObjGetArgs => {
            let mut args = args.into_inner();
            let id = Arg::UID(args.next().unwrap().as_str().parse().unwrap());

            return TaoArgs::OneArgs { arg1: id };
        }
        Rule::ObjDeleteArgs => {
            let mut args = args.into_inner();
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
fn unwrap_two_args(args: pest::iterators::Pair<Rule>) -> (&str, &str) {
    let mut args = args.into_inner();
    let a1 = args.next().unwrap().as_str();
    let a2 = args.next().unwrap().as_str();
    return (a1, a2);
}

fn unwrap_three_args(args: pest::iterators::Pair<Rule>) -> (&str, &str, &str) {
    let mut args = args.into_inner();
    let a1 = args.next().unwrap().as_str();
    let a2 = args.next().unwrap().as_str();
    let a3 = args.next().unwrap().as_str();

    return (a1, a2, a3);
}

fn unwrap_four_args(
    args: pest::iterators::Pair<Rule>,
) -> (&str, &str, &str, &str) {
    let mut args = args.into_inner();
    let a1 = args.next().unwrap().as_str();
    let a2 = args.next().unwrap().as_str();
    let a3 = args.next().unwrap().as_str();
    let a4 = args.next().unwrap().as_str();

    return (a1, a2, a3, a4);
}

fn unwrap_five_args(
    args: pest::iterators::Pair<Rule>,
) -> (&str, &str, &str, &str, &str) {
    let mut args = args.into_inner();
    let a1 = args.next().unwrap().as_str();
    let a2 = args.next().unwrap().as_str();
    let a3 = args.next().unwrap().as_str();
    let a4 = args.next().unwrap().as_str();
    let a5 = args.next().unwrap().as_str();

    return (a1, a2, a3, a4, a5);
}
fn unwrap_six_args(
    args: pest::iterators::Pair<Rule>,
) -> (&str, &str, &str, &str, &str, &str) {
    let mut args = args.into_inner();
    let a1 = args.next().unwrap().as_str();
    let a2 = args.next().unwrap().as_str();
    let a3 = args.next().unwrap().as_str();
    let a4 = args.next().unwrap().as_str();
    let a5 = args.next().unwrap().as_str();
    let a6 = args.next().unwrap().as_str();

    return (a1, a2, a3, a4, a5, a6);
}
