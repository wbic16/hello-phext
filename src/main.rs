#[macro_use] extern crate rocket;
use rocket::Request;
use rocket::http::Status;
mod phext;
mod phext_test;
use std::fs;

#[get("/<coordinate>")]
fn index(coordinate: &str) -> String {
  let parsed: phext::Coordinate = phext::to_coordinate(coordinate);

  let buffer:String = fs::read_to_string("world.phext").expect("Unable to find world");
  let scroll = phext::locate(&buffer, coordinate);
  return format!("{}", scroll);
}

#[catch(404)]
fn not_found(req: &Request) -> String {
  return format!("Unable to locate '{}'. Reach out to @wbic16 on twitter.", req.uri());
}

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    return format!("{} ({})", status, req.uri());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found, default])
        .mount("/", routes![index])
}