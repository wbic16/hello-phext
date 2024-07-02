#[macro_use] extern crate rocket;
mod phext;
mod phext_test;
use std::{fs::{self, File}, io::{Read, Write}};
use phext::Coordinate;
use rocket::Request;
use rocket::http::Status;
use rocket::http::ContentType;
use rocket::form::Form;

#[get("/api/<world>/<coordinate>")]
fn index(world: &str, coordinate: &str) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";
  let message = "Unable to find ".to_owned() + world;
  let buffer:String = fs::read_to_string(filename).expect(&message);
  let scroll = phext::locate(&buffer, coordinate);
  let response = "<html><head><title>Phext API Testing</title></head><body>".to_owned() + &scroll + "<br /><form method='POST'><input type='submit' value='Save' /><input type='hidden' name='world' value='" + world + "' /><input type='hidden' name='coordinate' value='1.1.1/1.1.1/3.2.1' /><br /><textarea rows='20' cols='110' name='scroll'>" + &scroll + "</textarea></form></body></html>";

  return (ContentType::HTML, response);
}


#[derive(Default, Debug, PartialEq, Eq, FromForm, Responder)]
struct Subspace {
    scroll: String,
}
impl Subspace {
  pub fn as_bytes(&self) -> &[u8] {
    return self.scroll.as_bytes();
  }
}

#[post("/api/<world>/<coordinate>", data="<scroll>")]
fn set(world: &str, coordinate: &str, scroll: Form<Subspace>) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";
  
  let prior = fs::read_to_string(filename.clone()).expect("Unable to open world");
  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;

  let message = phext::replace(prior.as_str(), phext::to_coordinate(coordinate), scroll.scroll.as_str());
  let _result = file.expect(&required).write_all(message.as_bytes());

  return (ContentType::HTML, "Wrote ".to_owned() + &filename + " at " + coordinate + ": " + scroll.scroll.as_str());
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
        .mount("/", routes![index, set])
}