use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::query::crypto::TaoCrypto;
use crate::query::query::TaoOp;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DBRow {
    AssocRow {
        id1: String,
        atype: String,
        id2: String,
        t: i64,
        data: String,
    },
    ObjRow {
        id: String,
        otype: String,
        data: String,
    },
    Count(i64),
    NoRes(bool),
}

fn deserialize_row(op: &TaoOp, row: &Row) -> DBRow {
    match op {
        TaoOp::AssocGet | TaoOp::AssocRangeGet | TaoOp::AssocRange => {
            DBRow::AssocRow {
                id1: row.get(0),
                atype: row.get(1),
                id2: row.get(2),
                t: row.get(3),
                data: row.get(4),
            }
        }
        TaoOp::AssocCount => DBRow::Count(row.get(0)),
        TaoOp::ObjGet => DBRow::ObjRow {
            id: row.get(0),
            otype: row.get(1),
            data: row.get(2),
        },
        _ => DBRow::NoRes(true),
    }
}

pub fn deserialize_rows(op: &TaoOp, rows: &Vec<Row>) -> Vec<DBRow> {
    let res = rows
        .iter()
        .map(|r| deserialize_row(op, r))
        .collect::<Vec<DBRow>>();
    return res;
}
