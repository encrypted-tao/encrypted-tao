use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum TaoOp {
    AssocGet,
    AssocRangeGet,
    AssocCount,
    AssocRange,
    AssocTimeRange,
    AssocAdd,
    AssocDelete,
    AssocChangeType,
    ObjAdd,
    ObjGet,
    ObjUpdate,
    ObjDelete,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Arg {
    ObjType(ObjType),
    AssocType(AssocType),
    Str(String),
    Num(u64),
    UID(u64),
    UIDSet(Vec<Arg>),
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arg::ObjType(ot) => write!(f, "{}", ot.to_string()),
            Arg::AssocType(at) => write!(f, "{}", at.to_string()),
            Arg::Str(s) => write!(f, "{}", s),
            Arg::Num(i) => write!(f, "{}", i.to_string()),
            Arg::UID(i) => write!(f, "{}", i.to_string()),
            Arg::UIDSet(_) => panic!("to_string for list not supported"),
        }
    }
}

#[derive(Debug)]
pub enum TaoArgs {
    OneArgs {
        arg1: Arg,
    },
    TwoArgs {
        arg1: Arg,
        arg2: Arg,
    },
    ThreeArgs {
        arg1: Arg,
        arg2: Arg,
        arg3: Arg,
    },
    FourArgs {
        arg1: Arg,
        arg2: Arg,
        arg3: Arg,
        arg4: Arg,
    },
    FiveArgs {
        arg1: Arg,
        arg2: Arg,
        arg3: Arg,
        arg4: Arg,
        arg5: Arg,
    },
}

#[derive(Debug)]
pub struct Query {
    pub op: TaoOp,
    pub args: TaoArgs,
}