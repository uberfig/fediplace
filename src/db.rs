use rocket::serde::Serialize;
use diesel::{self, result::QueryResult, prelude::*};

mod schema {
    table! {
        pallete {
            id -> Integer,
            r -> Integer,
            g -> Integer,
            b -> Integer,
        }
        users {
            db_id -> Integer,
            id -> Text,
            kind -> Text,
            preferred_username -> Text,
            name -> Text,
            inbox -> Text,
            outbox -> Text,
            public_key -> Text,
            last_placed -> Integer, //time they placed their last pixel
        }
        pixels {
            x -> Integer,
            y -> Integer,
            color -> Integer,
            user -> Integer,
            time -> Integer,
        }
    }
}

use self::schema;
use crate::DbConn;
