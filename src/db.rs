use mongodb::{Client, Database , options:: {ClientOptions, StreamAddress} };
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
use std::ops::Deref;

pub struct Conn(pub Database);

pub fn init() -> Database {
  let host = env::var("MONGO_HOST").expect("MONGO_HOST env not set.");    // TODO check if this is shit for performance
  let port = env::var("MONGO_PORT").expect("MONGO_PORT env not set.");    // TODO check if this is shit for performance
  let db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME env not set.");    // TODO check if this is shit for performance

  let options = ClientOptions::builder()
    .hosts(vec![
      StreamAddress {
        hostname: host,
        port: Some(port.parse::<u16>().unwrap()),
      }
    ])
    .build();

  match Client::with_options(options) {
    Ok(client) => client.database(&db_name),
    Err(e) =>  panic!("Error: failed to connect to mongodb {}", e),
  }
}

/*
    Create a implementation of FromRequest so Conn can be provided at every api endpoint
*/
impl<'a, 'r> FromRequest<'a, 'r> for Conn {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
    let database = request.guard::<State<Database>>()?;
    let clone = database.clone();
    Outcome::Success(Conn(clone))
  }
}

/*
    When Conn is dereferencd, return the mongo connection.
*/
impl Deref for Conn {
  type Target = Database;
  fn deref(&self) -> &Self::Target {
      &self.0
  }
}
