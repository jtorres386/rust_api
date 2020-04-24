use crate::heroes;
use crate::db::Conn;
use crate::errors::Error;
use heroes::Hero;
use bson::{ doc, oid::ObjectId };
use rocket_contrib::json::Json;
use rocket::{http::Status};

fn error_status(error: Error) -> Status {
  match error {
    Error::CursorNotFoundError => Status::NotFound,
    _ => Status::InternalServerError,
  }
}

#[get("/")]
pub fn all(connection: Conn) -> Result<Json<Vec<Hero>>, Status> {
  match heroes::controller::all(&connection) {
    Ok(res) => Ok(Json(serde_json::from_str(&(serde_json::to_string(&res).unwrap())).unwrap())),
    Err(err) => Err(error_status(err)),
  }
}

#[get("/<id>")]
pub fn get(id: String, connection: Conn) -> Result<Json<Hero>, Status> {
  match ObjectId::with_string(&String::from(&id)) {
    Ok(res) => match heroes::controller::get(res, &connection) {
      Ok(res) => Ok(Json(res.unwrap())),
      Err(err) => Err(error_status(err)),
    }
    Err(_) => Err(error_status(Error::DefaultError(String::from("Couldn't parse ObjectId"))))
  }
}

#[post("/", format = "application/json", data = "<hero>")]
pub fn post(hero: Json<Hero>, connection: Conn) -> Result<Json<ObjectId>, Status> {
  match heroes::controller::insert(hero.into_inner(), &connection) {
    Ok(res) => Ok(Json(res)),
    Err(err) => Err(error_status(err)),
  }
}

#[put("/<id>", format = "application/json", data = "<hero>")]
pub fn put(id: String, hero: Json<Hero>, connection: Conn) -> Result<Json<Hero>, Status> {
  match ObjectId::with_string(&String::from(&id)) {
    Ok(res) => match heroes::controller::update(res, hero.into_inner(), &connection) {
      Ok(res) => Ok(Json(res)),
      Err(err) => Err(error_status(err)),
    }
    Err(_) => Err(error_status(Error::DefaultError(String::from("Couldn't parse ObjectId"))))
  }
}

#[delete("/<id>")]
pub fn delete(id: String, connection: Conn) -> Result<Json<String>, Status> {
  match ObjectId::with_string(&String::from(&id)) {
    Ok(res) => match heroes::controller::delete(res, &connection) {
      Ok(_) => Ok(Json(id)),
      Err(err) => Err(error_status(Error::MongoError(err))),
    }
    Err(_) => Err(error_status(Error::DefaultError(String::from("Couldn't parse ObjectId"))))
  }
}

#[delete("/")]
pub fn delete_all(connection: Conn) -> Result<Json<bool>, Status> {
  match heroes::controller::delete_all(&connection) {
    Ok(_) => Ok(Json(true)),
    Err(err) => Err(error_status(Error::MongoError(err))),
  }
}