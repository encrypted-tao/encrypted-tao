use std::str::FromStr;

#[derive(Debug)]
pub enum TaoOp {
    AssocGet,
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
    Checkin,
}
impl FromStr for ObjType {
    type Err = ();

    fn from_str(input: &str) -> Result<ObjType, Self::Err> {
        match input {
            "USER" => Ok(ObjType::User),
            "COMMENT" => Ok(ObjType::Comment),
            "LOCATION" => Ok(ObjType::Location),
            "POST" => Ok(ObjType::Checkin),
            _ => Err(()),
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

#[derive(Debug)]
pub enum Arg {
    ObjType(ObjType),
    AssocType(AssocType),
    Str(String),
    Num(u64),
    UID(u64),
    UIDSet(Vec<Arg>),
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
    SixArgs {
        arg1: Arg,
        arg2: Arg,
        arg3: Arg,
        arg4: Arg,
        arg5: Arg,
        arg6: Arg,
    },
}

#[derive(Debug)]
pub struct Query {
    pub op: TaoOp,
    pub args: TaoArgs,
}
