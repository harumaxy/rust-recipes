use failure_derive::Fail;
use rocket::http::{ContentType, Status};

use rocket::response::{Responder, Response};
use std::io::Cursor;

#[derive(Fail, Debug)]
pub enum DoodleWebErr {
    #[fail(display = "IO Error{}", 0)]
    IOErr(std::io::Error),

    #[fail(display = "Database Error{}", 0)]
    DatabaseErr(diesel::result::Error),

    #[fail(display = "User does not exsit")]
    UserDoesNotExistErr,
    #[fail(display = "Password is invalid")]
    PasswordErr,
}

impl From<std::io::Error> for DoodleWebErr {
    fn from(e: std::io::Error) -> Self {
        DoodleWebErr::IOErr(e)
    }
}

impl From<diesel::result::Error> for DoodleWebErr {
    fn from(e: diesel::result::Error) -> Self {
        DoodleWebErr::DatabaseErr(e)
    }
}

impl<'r> Responder<'r> for DoodleWebErr {
    fn respond_to(self, request: &rocket::request::Request) -> rocket::response::Result<'r> {
        let res = Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::Plain)
            // sized_body response には、Seakable trait を実装したオブジェクトが必要
            .sized_body(Cursor::new(format!("Error doing loading page : {}", self)))
            .finalize();
        Ok(res)
    }
}
