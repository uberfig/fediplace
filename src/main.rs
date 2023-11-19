#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;

mod task;
#[cfg(test)]
mod tests;
use diesel::prelude::*;
use diesel::{sql_query, RunQueryDsl, QueryDsl};
use fediplace::{CreateActivity, Pixel, Actor, schema::pixels};
use parking_lot::Mutex;
use rocket::{
    fairing::AdHoc,
    form::Form,
    fs::{relative, FileServer},
    request::FlashMessage,
    response::{Flash, Redirect},
    serde::json::{self, Json},
    serde::{Deserialize, Serialize},
    // tokio::time::{sleep, Duration},
    Build, Rocket,
    response::status,
    State,
};

use rocket_dyn_templates::Template;

use crate::task::{Task, Todo};
use fediplace::DbConn;



#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Context {
    flash: Option<(String, String)>,
    tasks: Vec<Task>,
}

impl Context {
    pub async fn err<M: std::fmt::Display>(conn: &DbConn, msg: M) -> Context {
        Context {
            flash: Some(("error".into(), msg.to_string())),
            tasks: Task::all(conn).await.unwrap_or_default(),
        }
    }

    pub async fn raw(conn: &DbConn, flash: Option<(String, String)>) -> Context {
        match Task::all(conn).await {
            Ok(tasks) => Context { flash, tasks },
            Err(e) => {
                error_!("DB Task::all() error: {}", e);
                Context {
                    flash: Some(("error".into(), "Fail to access database.".into())),
                    tasks: vec![],
                }
            }
        }
    }
}

#[post("/", data = "<todo_form>")]
async fn new(todo_form: Form<Todo>, conn: DbConn) -> Flash<Redirect> {
    let todo = todo_form.into_inner();
    if todo.description.is_empty() {
        Flash::error(Redirect::to("/"), "Description cannot be empty.")
    } else if let Err(e) = Task::insert(todo, &conn).await {
        error_!("DB insertion error: {}", e);
        Flash::error(
            Redirect::to("/"),
            "Todo could not be inserted due an internal error.",
        )
    } else {
        Flash::success(Redirect::to("/"), "Todo successfully added.")
    }
}

#[put("/<id>")]
async fn toggle(id: i32, conn: DbConn) -> Result<Redirect, Template> {
    match Task::toggle_with_id(id, &conn).await {
        Ok(_) => Ok(Redirect::to("/")),
        Err(e) => {
            error_!("DB toggle({}) error: {}", id, e);
            Err(Template::render(
                "index",
                Context::err(&conn, "Failed to toggle task.").await,
            ))
        }
    }
}

#[delete("/<id>")]
async fn delete(id: i32, conn: DbConn) -> Result<Flash<Redirect>, Template> {
    match Task::delete_with_id(id, &conn).await {
        Ok(_) => Ok(Flash::success(Redirect::to("/"), "Todo was deleted.")),
        Err(e) => {
            error_!("DB deletion({}) error: {}", id, e);
            Err(Template::render(
                "index",
                Context::err(&conn, "Failed to delete task.").await,
            ))
        }
    }
}

// #[get("/")]
// async fn index(flash: Option<FlashMessage<'_>>, conn: DbConn) -> Template {
//     let flash = flash.map(FlashMessage::into_inner);
//     Template::render("index", Context::raw(&conn, flash).await)
// }

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    DbConn::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("diesel migrations");
        })
        .await;

    rocket
}

#[post("/@system/inbox", format = "json", data = "<create>")]
async fn inbox(create: Json<CreateActivity>, conn: DbConn, state: &State<CacheArr>) -> status::Accepted<String> {
    print!("recieved json data:");
    dbg!(&create);
    let works = Pixel::new_place(create.into_inner(), &conn).await;
    match works {
    Ok(fuck) => {
        state.inner()[fuck.x][fuck.y] = fuck.color;
        return status::Accepted(Some(format!("placed with x:{x}, y:{y}, color:{z}",x=fuck.x,y=fuck.y,z=fuck.color)))
    },
    Err(e) => {
        return status::Accepted(Some(format!("unable to parse err: {}", e)))
    },
}
}

#[get("/@system")]
async fn system() -> serde_json::Value {
    let a = Actor::system();
    let j = serde_json::json!(&a);
    return j;
}

// #[get("/canvas")]
// async fn canvas() -> Json<Canvas> {
//     Canvas{pallete: vec![Color{1,2,3}]}
// }

struct CacheArr{
    pub color: [[u8; 1024]; 1024],
    pub placer: Mutex<u8>,
}

async fn initArray(rocket: Rocket<Build>, conn: &DbConn) -> CacheArr {
    let b: [[u8; 1024]; 1024];

    let a = conn.run(|c| {
        pixels::table.order(pixels::id.desc()).load::<Pixel>(c)
    }).await;

    for i in a.expect("unable to load db") {
        
    }

    return CacheArr {color: b, placer: Mutex<RawMutex, u8>};
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .attach(AdHoc::on_liftoff("Run On Start", |_| {
            Box::pin(async {
                dbg!(fediplace::get_person("https://mastodon.social/@LemmyDev").await).unwrap();
            })
        }))
        .manage(CacheArr)
        // .mount("/", FileServer::from(relative!("static/public")))
        .mount("/", FileServer::from(relative!("static")))
        // .mount("/", routes![index])
        .mount("/todo", routes![new, toggle, delete])
        .mount("/", routes![inbox, system])
}
