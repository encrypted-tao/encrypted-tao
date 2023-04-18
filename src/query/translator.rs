use crate::query::query::{Arg, Query, SqlQuery, TaoArgs, TaoOp};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;

pub fn translate(query: Query) -> SqlQuery {
    let query = match query.op {
        TaoOp::AssocAdd => panic!("no"), 
        TaoOp::AssocGet => panic!("no"),
        TaoOp::AssocRangeGet => panic!("no"),
        TaoOp::AssocCount => panic!("no"),
        TaoOp::AssocRange => panic!("no"),
        TaoOp::AssocTimeRange => panic!("no"),
        TaoOp::AssocDelete => panic!("Not supported yet!"),
        TaoOp::AssocChangeType => panic!("Not supported yet!"),
        TaoOp::ObjAdd => translate_obj_add(query.args),
        TaoOp::ObjGet => translate_obj_get(query.args),
        TaoOp::ObjUpdate => panic!("Not supported yet!"),
        TaoOp::ObjDelete => panic!("Not supported yet!"),
    };
    return query;
}

/*
fn translate_assoc_add(args: TaoArgs) -> SqlQuery {
    match args {
        TaoArgs::FiveArgs {
            arg1: id1,
            arg2: atype,
            arg3: id2,
            arg4: time,
            arg5: data,
        } => {
            let query =
                "INSERT INTO Associations (id1, atype, id2, time, data) \
                         VALUES ($1, $2, $3, $4, $5)";
            /*
            let data = match data {
                Arg::Str(s) => s,
                _ => panic!("Invalid data arg"),
            };
            */
            let id1: ToSql = id1;
            let atype: ToSql = atype.to_string().as_str();
            let id2: ToSql = id2;
            let time: ToSql = time;
            let data: ToSql = data.as_str();

            return SqlQuery {
                op: TaoOp::AssocAdd,
                query: query.to_string(),
                params: &[&id1, &atype, &id2, &time, &data]
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
            // let idset = unwrap_idset(idset);
            let in_set = format_in_clause(&idset, 2);
            let query = format!(
                "SELECT * \
                 FROM Associations \
                 WHERE id1 = $1 \
                 AND atype = $2 \
                 AND id2 in {in_set}"
            );
            let mut params = vec![id, atype];
            /*
            let idset_str =
                idset.iter().map(|i| i).collect::<Vec<String>>();
            */
            // params.extend(idset_str);
            let idsett = match idset {
                Arg::UIDSet(us) => us,
                _ => panic!("Expected UIDSet"),
            };
            params.extend(idsett);
            return SqlQuery {
                op: TaoOp::AssocGet,
                query: query.to_string(),
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
            // let idset = unwrap_idset(idset);
            let in_set = format_in_clause(&idset, 4);
            let query = format!(
                "SELECT * \
                 FROM Associations \
                 WHERE id1 = $1 \
                   AND atype = $2 \
                   AND time >= $3 \
                   AND time <= $4 \
                   AND id2 in {in_set}"
            );

            let mut params = vec![
                id,
                atype,
                lo,
                hi,
            ];
            /* let idset_str =
                idset.iter().map(|i| i).collect::<Vec<String>>();
            */
            // params.extend(idset_str);
            let idsett = match idset {
                Arg::UIDSet(us) => us,
                _ => panic!("Expected UIDSet"),
            };
            params.extend(idsett);
            return SqlQuery {
                op: TaoOp::AssocRangeGet,
                query: query.to_string(),
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
            let params = vec![id, atype];

            return SqlQuery {
                op: TaoOp::AssocCount,
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
            let params = vec![id, atype];

            return SqlQuery {
                op: TaoOp::AssocRange,
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
                id,
                atype,
                t1,
                t2,
                lim,
            ];

            return SqlQuery {
                op: TaoOp::AssocTimeRange,
                query: query.to_string(),
                params: params,
            };
        }
        _ => panic!("Invalid arguments to ASSOC RANGE"),
    }
}
*/

fn translate_obj_add(args: TaoArgs) -> SqlQuery {
    match args {
        TaoArgs::ObjAddArgs {
            arg1: _,
            arg2: _,
            arg3: _,
        } => {
            let query = "INSERT INTO Objects (id, otype, data) \
                         VALUES ($1, $2, $3)";

            /*
            let data = match data {
                Arg::Str(s) => s,
                _ => panic!("Invalid data arg"),
            };
            */

            return SqlQuery {
                op: TaoOp::ObjAdd,
                query: query.to_string(),
                params: args,
            };
        }
        _ => panic!("Invalid arguments to OBJ ADD"),
    }
}

fn translate_obj_get(args: TaoArgs) -> SqlQuery {
    match args {
        TaoArgs::ObjGetArgs { arg1: _ } => {
            let query = "SELECT * \
                         FROM Objects \
                         WHERE id = $1";

            return SqlQuery {
                op: TaoOp::ObjGet,
                query: query.to_string(),
                params: args,
            };
        }
        _ => panic!("Invalid arguments to OBJ ADD"),
    }
}

// some crusty helper functions
fn format_in_clause(lst: &Arg, offset: i32) -> String {
    let lst = match lst {
        Arg::UIDSet(us) => us,
        _ => panic!("Expected UIDSet"),
    };
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
