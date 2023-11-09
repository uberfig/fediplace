// use rocket::serde::Serialize;
// use diesel::{self, result::QueryResult, prelude::*};

// mod schema {
//     table! {
//         pallete {
//             id -> Integer,
//             r -> Integer,
//             g -> Integer,
//             b -> Integer,
//         }
//         users {
//             db_id -> Integer,
//             id -> Text,
//             kind -> Text,
//             preferred_username -> Text,
//             name -> Text,
//             inbox -> Text,
//             outbox -> Text,
//             public_key -> Text,
//             last_placed -> Integer, //time they placed their last pixel
//         }
//         pixels {
//             x -> Integer,
//             y -> Integer,
//             color -> Integer,
//             user -> Integer,
//             time -> Integer,
//         }
//     }
// }

// use self::schema;
// use crate::DbConn;


// #[derive(Deserialize, Serialize)]
// pub struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
// }

// fn get_activity_fetch_client() -> &'static reqwest::Client{
//     static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();
//     CLIENT.get_or_init(||{
//         use reqwest::header;
//         let mut headers = header::HeaderMap::new();
//         headers.insert(reqwest::header::ACCEPT, header::HeaderValue::from_static("application/activity+json"));
//         Client::builder().default_headers(headers).build().unwrap()
//     })
// }


// pub async fn get_person(user: impl IntoUrl) -> reqwest::Result<Actor> {
//     get_activity_fetch_client().get(user).send().await?.json().await
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(crate = "rocket::serde")]
// enum PersonType {
//     Person,
//     Application,
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(crate = "rocket::serde")]
// #[serde(rename_all = "camelCase")]
// pub struct Actor {
//     id: Url,
//     #[serde(rename = "type")]
//     kind: PersonType,
//     preferred_username: String,
//     name: String,
//     inbox: Url,
//     outbox: Url,
//     public_key: public_key::PublicKey,
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(crate = "rocket::serde")]
// #[serde(rename_all = "camelCase")]
// pub struct Canvas {
//     pallete: Vec<Color>,
//     data: [u8],
// }