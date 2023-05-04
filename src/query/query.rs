use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaoOp {
    AssocGet,
    AssocRangeGet,
    AssocCount,
    AssocRange,
    AssocAdd,
    AssocDelete,
    AssocChangeType,
    ObjAdd,
    ObjGet,
    ObjUpdate,
    ObjDelete,
}

#[derive(Debug, Clone)]
pub enum ObjType {
    User,
    Comment,
    Location,
    Post,
}

impl FromStr for ObjType {
    type Err = ();

    fn from_str(input: &str) -> Result<ObjType, Self::Err> {
        match input {
            "USER" => Ok(ObjType::User),
            "COMMENT" => Ok(ObjType::Comment),
            "LOCATION" => Ok(ObjType::Location),
            "POST" => Ok(ObjType::Post),
            _ => Err(()),
        }
    }
}

impl fmt::Display for ObjType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjType::User => write!(f, "User"),
            ObjType::Comment => write!(f, "Comment"),
            ObjType::Location => write!(f, "Location"),
            ObjType::Post => write!(f, "Post"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AssocType {
    Friend,
    Loc,
    CheckIn,
    Comment,
    Authored,
    AuthoredBy,
    Likes,
    LikedBy,
}

impl FromStr for AssocType {
    type Err = ();

    fn from_str(input: &str) -> Result<AssocType, Self::Err> {
        match input {
            "FRIEND" => Ok(AssocType::Friend),
            "LOCATED" => Ok(AssocType::Loc),
            "COMMENT" => Ok(AssocType::Comment),
            "AUTHORED" => Ok(AssocType::Authored),
            "LIKES" => Ok(AssocType::Likes),
            _ => Err(()),
        }
    }
}

impl fmt::Display for AssocType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssocType::Friend => write!(f, "Friend"),
            AssocType::Loc => write!(f, "Loc"),
            AssocType::CheckIn => write!(f, "CheckIn"),
            AssocType::Comment => write!(f, "Comment"),
            AssocType::Authored => write!(f, "Authored"),
            AssocType::AuthoredBy => write!(f, "AuthoredBy"),
            AssocType::Likes => write!(f, "Likes"),
            AssocType::LikedBy => write!(f, "LikedBy"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TaoArgs {
    AssocAddArgs {
        id1: i64,
        atype: String,
        id2: i64,
        time: i64,
        data: String,
    },
    AssocGetArgs {
        id: i64,
        atype: String,
        idset: Vec<i64>,
    },
    AssocRangeGetArgs {
        id: i64,
        atype: String,
        idset: Vec<i64>,
        tstart: i64,
        tend: i64,
    },
    AssocCountArgs {
        id: i64,
        atype: String,
    },
    AssocRangeArgs {
        id: i64,
        atype: String,
        tstart: i64,
        tend: i64,
        lim: i64,
    },
    ObjGetArgs {
        id: i64,
    },
    ObjAddArgs {
        id: i64,
        otype: String,
        data: String,
    },
}

#[derive(Debug, Clone)]
pub struct Query {
    pub op: TaoOp,
    pub args: TaoArgs,
}

#[derive(Debug, Clone)]
pub struct SqlQuery {
    pub op: TaoOp,
    pub query: String,
    pub params: TaoArgs,
}

// some crusty helper functions that probably needs a better home
pub fn format_in_clause(lst: &Vec<i64>, offset: i32) -> String {
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
