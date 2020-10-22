#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use d6_doodle::{models::*, schema::*};
use diesel::prelude::*;
use rocket::request::Form;

pub mod error;
use crate::error::DoodleWebErr;

use rocket::response::{NamedFile, Responder};
use std::path::PathBuf;

use maud::{html, DOCTYPE};

#[get("/")]
fn root() -> Result<impl Responder<'static>, failure::Error> {
    NamedFile::open("site/static/index.html").map_err(|e| e.into())
}

#[get("/<path..>")]
fn static_file(path: PathBuf) -> Result<impl Responder<'static>, DoodleWebErr> {
    let path = PathBuf::from("site/static").join(path);
    NamedFile::open(path).map_err(|e| e.into())
}

#[database("doodlebase")]
pub struct DPool(diesel::pg::PgConnection);

#[derive(FromForm)]
pub struct LoginData {
    name: String,
    pass: String,
}

use rocket::http::{Cookie, Cookies};
use rocket::State;

#[post("/login", data = "<dt>")]
fn login(
    dt: Form<LoginData>,
    db: DPool,
    ss: State<session::Session>,
    mut cookies: Cookies,
) -> Result<impl Responder<'static>, DoodleWebErr> {
    let ldform = dt.into_inner();
    let vals = users::table
        .filter(users::name.eq(ldform.name))
        .load::<User>(&db.0)?;

    let user = vals
        .into_iter()
        .next()
        .ok_or(DoodleWebErr::UserDoesNotExistErr)?;
    if !user.verify_pass(&ldform.pass) {
        return Err(DoodleWebErr::PasswordErr);
    }

    let sess_id = ss.put(user.clone());
    cookies.add(Cookie::new("login", sess_id.to_string()));

    let res = html! {
        (DOCTYPE)
        head {
            meta charset = "utf-8";
        }
        body {
            h1 {"Welcome " ("Guest")}
            h2 { "Ask a question" }
            div style="border:1px solid black;" {
                form action ="question" method="POST"{
                    "Question" input type="text" name="question";
                    "Options" input type="text" name="options";
                    input type="submit" value="Ask Question";
                }
            }
        }

    };

    Ok(res)
}

pub mod session;

fn main() {
    let sess = session::Session::new();
    rocket::ignite()
        .mount("/", routes![root, static_file, login])
        .attach(DPool::fairing())
        .manage(sess)
        .launch();
}
