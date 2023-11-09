pub mod protocol;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;

use reqwest::Client;
use reqwest::IntoUrl;
use rocket::{
    fairing::AdHoc,
    // serde::{Deserialize, Serialize},
    form::Form,
    fs::{relative, FileServer},
    request::FlashMessage,
    response::{Flash, Redirect},
    serde::json::Json,
    tokio::time::{sleep, Duration},
    Build,
    Rocket,
};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use url::Url;
// use chrono::DateTime;
use chrono::Utc;
use protocol::public_key;
// use rocket::DbConn;

// use crate::DbConn;
#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

// use rocket::serde::Serialize;
use diesel::{result::QueryResult, prelude::*};
mod schema {
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
            time -> Integer,
        }
    }
}

use self::schema::{
    pallete,
    users,
    pixels,
};


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
enum PersonType {
    Person,
    Application,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    id: Url,
    #[serde(rename = "type")]
    kind: PersonType,
    preferred_username: String,
    name: String,
    inbox: Url,
    outbox: Url,
    public_key: public_key::PublicKey,
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
    object: object,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct object {
    id: Url,
    #[serde(rename = "type")]
    kind: ActivityType,
    content: String,
}

enum PixelCreator {
    Federated(Actor),
    System,
}

#[derive(Queryable, Debug, Clone)]
// #[serde(crate = "rocket::serde")]
#[diesel(table_name = pixels)]
pub struct Pixel {
    id: u32,
    x: u16,
    y: u16,
    color: u8,
    user: u32,
    time: chrono::DateTime<chrono::Utc>,
}

impl Pixel {
    pub async fn new_place(activity: CreateActivity, conn: &DbConn) {
        let bruh = activity.object.content.split_ascii_whitespace();
        let x: Vec<&str> = bruh.collect();
        // conn.run(|c| {
        //     let t = Pixel { id: None, description: todo.description, completed: false };
        //     diesel::insert_into(tasks::table).values(&t).execute(c)
        // }).await
    }
}

// pub fn get_pixel(x: u16, y: u16) -> Pixel {
//     return Pixel::newdebug();
// }
