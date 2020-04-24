// See https://rocket.rs/v0.4/guide/testing/#local-dispatching
#[cfg(test)]
mod test {
  use rocket::http::{ContentType, Status};
  use rocket::local::Client;
  use rustlang_rocket_mongodb::rocket;

  #[test]
  fn get_heroes() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/heroes").dispatch();
    assert_eq!(response.status(), Status::Ok);
  }

  #[test]
  fn get_heroe() {
    // Well get and post tests are identical ...
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
      .post("/heroes")
      .header(ContentType::JSON)
      .body(r#"{ "name": "Super Test", "identity":"My Name", "hometown":"cdmx", "age":30 }"#)
      .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let id = response.body_string().unwrap();
    let id: Vec<&str> = id.split("\"").collect();
    let mut response = client.get(format!("/heroes/{}", id[3])).dispatch();
    println!("get_heroe: {:#?}", response);
    assert!(response.body().is_some());
    assert!(response.body_string().unwrap().contains(&id[3]));
    client.delete("/heroes").dispatch();
  }

  #[test]
  fn post_heroe() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
      .post("/heroes")
      .header(ContentType::JSON)
      .body(r#"{ "name": "Super Test", "identity":"My Name", "hometown":"cdmx", "age":30 }"#)
      .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let id = response.body_string().unwrap();
    let id: Vec<&str> = id.split("\"").collect();
    let mut response = client.get(format!("/heroes/{}", id[3])).dispatch();
    assert!(response.body().is_some());
    assert!(response.body_string().unwrap().contains(&id[3]));
    client.delete("/heroes").dispatch();
  }

  #[test]
  fn update_heroe() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
      .post("/heroes")
      .header(ContentType::JSON)
      .body(r#"{ "name": "Super Test", "identity":"My Name", "hometown":"cdmx", "age":30 }"#)
      .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
    let id = response.body_string().unwrap();
    let id: Vec<&str> = id.split("\"").collect();
    let response = client
      .put(format!("/heroes/{}", id[3]))
      .header(ContentType::JSON)
      .body(r#"{ "name": "Super Second", "identity":"Mr John", "hometown":"cdmx", "age":10 }"#)
      .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut response = client.get(format!("/heroes/{}", id[3])).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
    assert!(response.body_string().unwrap().contains("Super Second"));
    client.delete("/heroes").dispatch();
  }

  #[test]
  fn delete_heroe() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
      .post("/heroes")
      .header(ContentType::JSON)
      .body(r#"{ "name": "Super Second", "identity":"Mr John", "hometown":"cdmx", "age":10 }"#)
      .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let id = response.body_string().unwrap();
    let id: Vec<&str> = id.split("\"").collect();
    let mut response = client.delete(format!("/heroes/{}", id[3])).dispatch();
    assert!(response.body().is_some());
    assert!(response.body_string().unwrap().contains(&id[3]));
    client.delete("/heroes").dispatch();
  }

}