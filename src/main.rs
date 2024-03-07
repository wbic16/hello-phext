#[macro_use] extern crate rocket;
use rocket::Request;
use rocket::http::Status;
mod phext;
mod phext_test;
use std::fs;

#[get("/api/<world>/<coordinate>")]
fn index(world: &str, coordinate: &str) -> String {
  let filename = world.to_owned() + ".phext";
  let parsed: phext::Coordinate = phext::to_coordinate(coordinate);

  let message = "Unable to find ".to_owned() + world;
  let buffer:String = fs::read_to_string(filename).expect(&message);
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