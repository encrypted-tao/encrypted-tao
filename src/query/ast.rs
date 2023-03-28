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

pub enum ObjType {
    User,
    Comment,
    Location,
    Checkin
}

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

pub enum ArgType {
    ObjType,
    AssocType,
    Str(String),
    Num(u64), 
}

pub enum Query {
    TwoArgs {
        op: TaoOp,
        arg1: ArgType,
        arg2: ArgType,
    },
    ThreeArgs {
        op: TaoOp,
        arg1: ArgType,
        arg2: ArgType,
        arg3: ArgType,
    },
    FourArgs {
        op: TaoOp,
        arg1: ArgType,
        arg2: ArgType,
        arg3: ArgType,
        arg4: ArgType,
    },
    FiveArgs {
        op: TaoOp,
        arg1: ArgType,
        arg2: ArgType,
        arg3: ArgType,
        arg4: ArgType,
        arg5: ArgType,
    },
}
