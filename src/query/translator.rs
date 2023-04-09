use crate::query::query::{Arg, Query, SqlQuery, TaoArgs, TaoOp};
use serde::{Deserialize, Serialize};

pub fn translate(query: Query) -> SqlQuery {
    let query = match query.op {
        TaoOp::AssocAdd => translate_assoc_add(query.args),
        TaoOp::AssocGet => translate_assoc_get(query.args),
        TaoOp::AssocRangeGet => translate_assoc_range_get(query.args),
        TaoOp::AssocCount => translate_assoc_count(query.args),
        TaoOp::AssocRange => translate_assoc_range(query.args),
        TaoOp::AssocTimeRange => translate_assoc_time_range(query.args),
        TaoOp::AssocDelete => panic!("Not supported yet!"),
        TaoOp::AssocChangeType => panic!("Not supported yet!"),
        TaoOp::ObjAdd => translate_obj_add(query.args),
        TaoOp::ObjGet => translate_obj_get(query.args),
        TaoOp::ObjUpdate => panic!("Not supported yet!"),
        TaoOp::ObjDelete => panic!("Not supported yet!"),
    };
    return query;
}

fn translate_assoc_add(args: TaoArgs) -> SqlQuery {
    match args {
        TaoArgs::FiveArgs {
            arg1: id1,
            arg2: atype,
            arg3: id2,
            arg4: t,
            arg5: data,
        } => {
            let query =
                "INSERT INTO Associations(id1, atype, id2, time, data) \
                         VALUES ($1, $2, $3, $4, $5)";
            let data = match data {
                Arg::Str(s) => s,
                _ => panic!("Invalid data arg"),
            };
            let params = vec![
                id1.to_string(),
                atype.to_string(),
                id2.to_string(),
                t.to_string(),
                data,
            ];

            return SqlQuery {
                query: query.to_string(),
                params: params,
            };
        }
        _ => panic!("Invalid arguments to ASSOC ADD"),
    }
}

fn translate_assoc_get(args: TaoArgs) -> SqlQuery {
    match args {
        TaoArgs::ThreeArgs {
            arg1: id,
            arg2: atype,
            arg3: idset,
        } => {
            let idset = unwrap_idset(idset);
            let in_set = format_in_clause(&idset, 2);
            let query = format!(
                "SELECT id1, id2, time, data \
                 FROM Associations \
                 WHERE id1 = $1 \
                 AND atype = $2 \
                 AND id2 in {in_set}"
            );
            let mut params = vec![id.to_string(), atype.to_string()];
            let idset_str =
                idset.iter().map(|i| i.to_string()).collect::<Vec<String>>();
            params.extend(idset_str);
            return SqlQuery {
                query: query,
                params: params,
            };
        }
        _ => panic!("Invalid arguments to ASSOC GET"),
    }
}

fn translate_assoc_range_get(args: TaoArgs) -> SqlQuery {
    match args {
        TaoArgs::FiveArgs {
            arg1: id,
            arg2: atype,
            arg3: idset,
            arg4: hi,
            arg5: lo,
        } => {
            let idset = unwrap_idset(idset);
            let in_set = format_in_clause(&idset, 4);
            let query = format!(
                "SELECT id1, id2, time, data \
                 FROM Associations \
                 WHERE id1 = $1 \
                   AND atype = $2 \
                   AND time >= $3 \
                   AND time <= $4 \
                   AND id2 in {in_set}"
            );

            let mut params = vec![
                id.to_string(),
                atype.to_string(),
                lo.to_string(),
                hi.to_string(),
            ];
            let idset_str =
                idset.iter().map(|i| i.to_string()).collect::<Vec<String>>();
            params.extend(idset_str);
            return SqlQuery {
                query: query,
                params: params,
            };
        }
        _ => panic!("Invalid arguments to ASSOC GET"),
    }
}

fn translate_assoc_count(args: TaoArgs) -> SqlQuery {
    match args {
        TaoArgs::TwoArgs {
            arg1: id,
            arg2: atype,
        } => {
            let query = "SELECT COUNT(*) \
                         FROM Associations \
                         WHERE id1 = $1 \
                           AND atype = $2";
            let params = vec![id.to_string(), atype.to_string()];

            return SqlQuery {
                query: query.to_string(),
                params: params,
            };
        }
        _ => panic!("Invalid arguments to ASSOC COUNT"),
    }
}

fn translate_assoc_range(args: TaoArgs) -> SqlQuery {
    match args {
        TaoArgs::FourArgs {
            arg1: id,
            arg2: atype,
            arg3: _,
            arg4: _,
        } => {
            let query = "SELECT * \
                         FROM Associations \
                         WHERE id1 = $1 \
                           AND atype = $2";
            let params = vec![id.to_string(), atype.to_string()];

            return SqlQuery {
                query: query.to_string(),
                params: params,
            };
        }
        _ => panic!("Invalid arguments to ASSOC RANGE"),
    }
}

fn translate_assoc_time_range(args: TaoArgs) -> SqlQuery {
    match args {
        TaoArgs::FiveArgs {
            arg1: id,
            arg2: atype,
            arg3: t1,
            arg4: t2,
            arg5: lim,
        } => {
            let query = "SELECT * \
                         FROM Associations \
                         WHERE id1 = $1 \
                           AND atype = $2 \
                           AND time >= $3 \
                           AND time <= $4 \
                         ORDER time DESC \
                         LIMIT $5";
            let params = vec![
                id.to_string(),
                atype.to_string(),
                t1.to_string(),
                t2.to_string(),
                lim.to_string(),
            ];

            return SqlQuery {
                query: query.to_string(),
                params: params,
            };
        }
        _ => panic!("Invalid arguments to ASSOC RANGE"),
    }
}

fn translate_obj_add(args: TaoArgs) -> SqlQuery {
    match args {
        TaoArgs::ThreeArgs {
            arg1: id,
            arg2: otype,
            arg3: data,
        } => {
            let query = "INSERT INTO Objects(id, otype, data) \
                         VALUES ($1, $2, $3)";
            let data = match data {
                Arg::Str(s) => s,
                _ => panic!("Invalid data arg"),
            };
            let params = vec![id.to_string(), otype.to_string(), data];

            return SqlQuery {
                query: query.to_string(),
                params: params,
            };
        }
        _ => panic!("Invalid arguments to OBJ ADD"),
    }
}

fn translate_obj_get(args: TaoArgs) -> SqlQuery {
    match args {
        TaoArgs::OneArgs { arg1: id } => {
            let query = "SELECT * \
                         FROM Objects \
                         WHERE id = $1";
            let params = vec![id.to_string()];

            return SqlQuery {
                query: query.to_string(),
                params: params,
            };
        }
        _ => panic!("Invalid arguments to OBJ ADD"),
    }
}

// some crusty helper functions
fn format_in_clause(lst: &Vec<Arg>, offset: i32) -> String {
    let sz = lst.len() as i32;
    let indices = (1..(sz + 1))
        .map(|i| {
            let v = i + offset;
            format!("${v}")
        })
        .collect::<Vec<String>>();
    let tup = indices.join(", ");
    return format!("({tup})");
}

fn unwrap_idset(idset: Arg) -> Vec<Arg> {
    match idset {
        Arg::UIDSet(us) => return us,
        _ => panic!("Expected Vec<Arg>"),
    }
}
