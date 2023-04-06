pub struct Object {
    pub id: i32,
    pub key: String,
    pub obj_type: String,
    pub val: String,
}

pub let OBJECTS: &[Object] = &[
    Object {
        id :1,
        key: "Alice",
        obj_type: "User",
        val: "",
    },
    Object {
        id :2,
        key: "Cathy",
        obj_type: "User",
        val: "",
    },
    Object {
        id :3,
        key: "Golden Gate",
        obj_type: "Location",
        val: "",
    },
    Object {
        id :4,
        key: "Checkin 1",
        obj_type: "Checkin",
        val: "",
    },
    Object {
        id :5,
        key: "Checkin 2",
        obj_type: "Checkin",
        val: "",
    },
];

pub struct Association {
    pub id: i32,
    pub obj_1: i32,
    pub obj_2: i32,
    pub assoc_type: String,
    pub time_stamp: String,
    pub key: String,
    pub val: String,
}

pub const ASSOCIATIONS: &[Association] = &[
    Association {
        id :1,
        obj_1: 1,
        obj_2: 2,
        assoc_type: "Friend",
        time_stamp: "2022-03-25 14:00:00",
        key: "",
        val: "",
    },
    Association {
        id :2,
        obj_1: 2,
        obj_2: 1,
        assoc_type: "Friend",
        time_stamp: "2022-03-25 14:00:00",
        key: "",
        val: "",
    },
    Association {
        id :3,
        obj_1: 3,
        obj_2: 4,
        assoc_type: "Location",
        time_stamp: "2022-03-25 15:00:00",
        key: "",
        val: "",
    },
    Association {
        id :4,
        obj_1: 4,
        obj_2: 3,
        assoc_type: "Checkin",
        time_stamp: "2022-03-25 15:00:00",
        key: "",
        val: "",
    },
    Association {
        id :5,
        obj_1: 5,
        obj_2: 3,
        assoc_type: "Checkin",
        time_stamp: "2022-03-26 10:00:00",
        key: "",
        val: "",
    },
];