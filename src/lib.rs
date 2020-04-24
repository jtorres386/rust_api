#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate mongodb;
extern crate rocket_contrib;
// #[macro_use]
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use rocket::{Request, Rocket};

pub mod logging;
pub mod errors;
pub mod heroes;
mod db;

#[catch(500)]
fn internal_error() -> &'static str {
  "Whoops! Looks like we messed up."
}

#[catch(400)]
fn not_found(req: &Request) -> String {
  format!("I couldn't find '{}'. Try something else?", req.uri())
}

pub fn rocket() -> Rocket {
  dotenv().ok();
  logging::init();
  rocket::ignite()
    .register(catchers![internal_error, not_found])
    .manage(db::init())
    .mount(
      "/heroes",
      routes![
        heroes::routes::all,
        heroes::routes::get,
        heroes::routes::post,
        heroes::routes::put,
        heroes::routes::delete,
        heroes::routes::delete_all
      ],
    )
}