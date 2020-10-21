#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;

pub fn create_connection() -> Result<PgConnection, failure::Error> {
    dotenv::dotenv().ok();
    let url = &std::env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(url)?)
}

#[cfg(test)]
mod tests {}
