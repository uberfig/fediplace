#![recursion_limit="512"]

pub mod protocol;

// #[macro_use]
// extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;

// use std::time::{SystemTime, UNIX_EPOCH};
// use chrono::DateTime;
// use chrono::NaiveDateTime;
use reqwest::Client;
use reqwest::IntoUrl;
// use rocket::data;
// use rocket::{
//     fairing::AdHoc,
//     // serde::{Deserialize, Serialize},
//     form::Form,
//     fs::{relative, FileServer},
//     request::FlashMessage,
//     response::{Flash, Redirect},
//     serde::json::Json,
//     tokio::time::{sleep, Duration},
//     Build,
//     Rocket,
// };
use serde::{Deserialize, Serialize};
// use std::net::IpAddr;
use url::Url;
// use chrono::DateTime;
// use chrono::Utc;
use protocol::public_key;
// use rocket::DbConn;

// use crate::DbConn;
#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

// use rocket::serde::Serialize;
use diesel::{prelude::*, result::QueryResult};
pub mod schema {
    table! {
        pallete {
            id -> Integer,
            r -> Integer,
            g -> Integer,
            b -> Integer,
        }

    }
    table! {
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
    }
    table! {
        pixels {
            id -> Integer,
            x -> Integer,
            y -> Integer,
            color -> Integer,
            user -> Integer,
            // insert_time -> Timestamp,
        }
    }
}

// use self::schema::{pallete::dsl::pallete, pixels::dsl::pixels, users::dsl::users};
use self::schema::pixels;
// use self::schema::pixels::dsl::pixels;

#[derive(Deserialize, Serialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn get_activity_fetch_client() -> &'static reqwest::Client {
    static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();
    CLIENT.get_or_init(|| {
        use reqwest::header;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            header::HeaderValue::from_static("application/activity+json"),
        );
        Client::builder().default_headers(headers).build().unwrap()
    })
}

pub async fn get_person(user: impl IntoUrl) -> reqwest::Result<Actor> {
    get_activity_fetch_client()
        .get(user)
        .send()
        .await?
        .json()
        .await
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum PersonType {
    Person,
    Application,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    pub id: Url,
    #[serde(rename = "type")]
    pub kind: PersonType,
    pub preferred_username: String,
    pub name: String,
    pub inbox: Url,
    pub outbox: Url,
    pub public_key: Option<public_key::PublicKey>,
}
impl Actor {
    pub fn system() -> Self {
        let actorid = format!("{host}/{id}\n", host = "http://localhost:8000", id = "@system");
        let actorinbox = format!("{host}/{id}\n", host = "http://localhost:8000", id = "@system/inbox");
        let actoroutbox = format!("{host}/{id}\n", host = "http://localhost:8000", id = "@system/outbox");
        Actor { id: Url::parse(&actorid).unwrap(), kind: PersonType::Person, 
        preferred_username: String::from("system"), name: String::from("fediplace"), inbox: Url::parse(&actorinbox).unwrap(), outbox: Url::parse(&actoroutbox).unwrap(), public_key: None }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
enum ActivityType {
    Create,
    Note,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct CreateActivity {
    id: Url,
    actor: Url,
    #[serde(rename = "type")]
    kind: ActivityType,
    object: Object,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct Object {
    id: Url,
    #[serde(rename = "type")]
    kind: ActivityType,
    content: String,
}

// enum PixelCreator {
//     Federated(Actor),
//     System,
// }

#[derive(Serialize, Queryable, Insertable, Debug, Clone, Selectable)]
// #[serde(crate = "rocket::serde")]
#[diesel(table_name = pixels)]
pub struct Pixel {
    id: i32,
    x: i32,
    y: i32,
    color: i32,
    user: i32,
    // insert_time: SystemTime,
}

#[derive(Debug, Clone)]
pub struct Fucky {
    pub string: String,
    pub x: u16,
    pub y: u16,
    pub color: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuckyParseError{
    WrongNumberOfArgs,
    InvalidVal1,
    InvalidVal2,
    InvalidVal3,
    InvalidVal4,
}


impl std::fmt::Display for FuckyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       match self {
        FuckyParseError::WrongNumberOfArgs => write!(f, "Incorrect Number Of args"),
        FuckyParseError::InvalidVal1 => write!(f, "somehow you managed to break the leading string >~<"),
        FuckyParseError::InvalidVal2 => write!(f, "X value is invalid"),
        FuckyParseError::InvalidVal3 => write!(f, "Y value is invalid"),
        FuckyParseError::InvalidVal4 => write!(f, "Color value is invalid"),
    }
    }
}

pub fn parse_fucky(data: &str) -> Result<Fucky, FuckyParseError> {
    use FuckyParseError as E;
    let vals: [&str; 4] = data
        .split_whitespace()
        // this is an uneeded allocation but it makes code easier
        .collect::<Vec<&str>>()
        .try_into()
        .map_err(|_| E::WrongNumberOfArgs)?;

    Ok(Fucky {
        string: vals[0].to_string(),
        x: vals[1].parse().map_err(|_|E::InvalidVal1)?,
        y: vals[2].parse().map_err(|_|E::InvalidVal2)?,
        color: vals[3].parse().map_err(|_|E::InvalidVal3)?,
        // val4: vals[4].parse().map_err(|_|E::InvalidVal4)?,
    })
}

impl Pixel {
    pub async fn new_place(activity: CreateActivity, conn: &DbConn) -> Result<Fucky, FuckyParseError> {
        // use FuckyParseError as E;
        let bruh = parse_fucky(&activity.object.content);
        let data;
        match bruh {
            Ok(x) => data = x,
            Err(y) => return Err(y),
        }
        let mut id: u32 = data.x.into();
        id = id << 16;
        id = id | data.y as u32;

        // dbg!(&data);
        let x = data.x as i32;
        let y = data.y as i32;
        let color = data.color as i32;

        let _dberr = conn.run(move |c| {
            let p = Pixel { id: id as i32, x: x, y: y, color: color, user: 1, /* insert_time: SystemTime::now().as_sql() */};
            diesel::insert_into(pixels::dsl::pixels).values(&p).execute(c)
        }).await;
        // dbg!(dberr);
        dbg!(&data);
        Ok(data)
    }
    // pub async fn insert(todo: Todo, conn: &DbConn) -> QueryResult<usize> {
    //     conn.run(|c| {
    //         let t = Task { id: None, description: todo.description, completed: false };
    //         diesel::insert_into(tasks::table).values(&t).execute(c)
    //     }).await
    // }
}

// pub fn get_pixel(x: u16, y: u16) -> Pixel {
//     return Pixel::newdebug();
// }
