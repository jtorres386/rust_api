#![allow(proc_macro_derive_resolution_fallback)]
use crate::heroes::{Hero, InsertableHero};
use crate::db::Conn;
use crate::errors::Error;
use bson::{ doc, oid::ObjectId };
use mongodb::results::{ DeleteResult };

const COLLECTION: &str = "hero";

use log::{info, warn};

pub fn all(database: &Conn) -> Result<Vec<Hero>, Error> {
  let cursor = database.collection(COLLECTION).find(None, None).unwrap();

  cursor
  .map(|result| match result {
    Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
      Ok(result_model) => Ok(result_model),
      Err(_) => Err(Error::DefaultError(String::from(""))),
    },
    Err(err) => Err(Error::MongoError(err)),
  })
  .collect::<Result<Vec<Hero>, Error>>()
}

pub fn get(id: ObjectId, database: &Conn) -> Result<Option<Hero>, Error> {
  match database.collection(COLLECTION).find_one(Some(doc! {"_id": id}), None) {
    Ok(db_result) => {
      println!("Document: {:#?}", db_result);
      match db_result {
        Some(result_doc) => {
          println!("Document: {:#?}", result_doc);
          match bson::from_bson(bson::Bson::Document(result_doc)) {
            Ok(result_model) => {
              match ::serde_json::to_string_pretty(&result_model) {
                Ok(txt) => info!("Result Model: {}", txt),
                Err(err) => warn!("Error: {}", err),
              }
              Ok(Some(result_model))
            },
            Err(_) => Err(Error::DefaultError(String::from("Failed to create reverse BSON"))),
          }
        },
        None => Ok(None),
      }
    },
    Err(err) => Err(Error::MongoError(err)),
  }
}

pub fn insert(hero: Hero, database: &Conn) -> Result<ObjectId, Error> {
  let insertable = InsertableHero::from_hero(hero.clone());
  match ::serde_json::to_string_pretty(&insertable) {
    Ok(txt) => info!("InsertableHero: {}", txt),
    Err(err) => warn!("InsertableHero ERROR: {}", err),
  }

  match bson::to_bson(&insertable) {
    Ok(model_bson) => match model_bson {
      bson::Bson::Document(model_doc) => {
        println!("Document: {:#?}", model_doc);
        match database.collection(COLLECTION).insert_one(model_doc, None) {
          Ok(res) => {
            println!("insertOne: {:#?}", res);
            info!("insertOne: {}", res.inserted_id);
            match bson::from_bson(res.inserted_id) {
              Ok(res) => Ok(res),
              Err(_) => Err(Error::DefaultError(String::from("Failed to read BSON")))
            }
          },
          Err(err) => Err(Error::MongoError(err)),
        }
      }
      _ => Err(Error::DefaultError(String::from("Failed to create Document"))),
    },
    Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
  }
}

pub fn update(id: ObjectId, hero: Hero, database: &Conn) -> Result<Hero, Error> {
  let mut new_hero = hero.clone();
  new_hero.id = Some(id.clone());
  match bson::to_bson(&new_hero) {
    Ok(model_bson) => match model_bson {
      bson::Bson::Document(model_doc) => {
        match database.collection(COLLECTION)
          .replace_one(doc! {"_id": id}, model_doc, None)
        {
          Ok(_) => Ok(new_hero),
          Err(err) => Err(Error::MongoError(err)),
        }
      }
      _ => Err(Error::DefaultError(String::from(
        "Failed to create Document",
      ))),
    },
    Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
  }
}

pub fn delete(id: ObjectId, database: &Conn) -> Result<DeleteResult, mongodb::error::Error> {
  database.collection(COLLECTION).delete_one(doc! {"_id": id}, None)
}

pub fn delete_all(database: &Conn) -> Result<(), mongodb::error::Error> {
  database.collection(COLLECTION).drop(None)
}
