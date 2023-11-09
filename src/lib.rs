pub mod protocol;

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

pub struct Pixel {
    color: Color,
    placed_by: PixelCreator,
    time: chrono::DateTime<chrono::Utc>,
}

impl Pixel {
    fn newdebug() -> Pixel {
        return Pixel {
            color: Color {
                r: 255,
                g: 255,
                b: 255,
            },
            placed_by: PixelCreator::System,
            time: Utc::now(),
        };
    }
}

// pub fn get_pixel(x: u16, y: u16) -> Pixel {
//     return Pixel::newdebug();
// }
