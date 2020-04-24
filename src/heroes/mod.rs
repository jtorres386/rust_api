#![allow(proc_macro_derive_resolution_fallback)]

pub mod routes;
pub mod controller;
use serde::{Serialize, Deserialize};
use bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hero {
  #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
  pub id: Option<bson::oid::ObjectId>,
  pub name: String,
  pub identity: String,
  pub hometown: String,
  pub age: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableHero {
  pub name: String,
  pub identity: String,
  pub hometown: String,
  pub age: i32
}

impl InsertableHero {
  fn from_hero(hero: Hero) -> InsertableHero {
    InsertableHero {
      name: hero.name,
      identity: hero.identity,
      hometown: hero.hometown,
      age : hero.age
    }
  }
}
