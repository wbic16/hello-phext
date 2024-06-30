#[macro_use] extern crate rocket;
mod phext;
mod phext_test;
use std::{fs::{self, File}, io::Write};
use rocket::{data::ToByteUnit, Request};
use rocket::http::Status;
use rocket::http::ContentType;
use rocket::tokio::io::AsyncReadExt;
use rocket::outcome::Outcome::Success;

#[get("/api/<world>/<coordinate>")]
fn index(world: &str, coordinate: &str) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";
  let message = "Unable to find ".to_owned() + world;
  let buffer:String = fs::read_to_string(filename).expect(&message);
  let scroll = phext::locate(&buffer, coordinate);
  let response = "<html><head><title>Test</title></head><body>".to_owned() + &scroll + "<br /><form method='POST'><input type='submit' value='Save' /><input type='hidden' name='world' value='" + world + "' /><input type='hidden' name='coordinate' value='1.1.1/1.1.1/3.2.1' /><br /><textarea rows='20' cols='110' name='scroll'>" + &scroll + "</textarea></form></body></html>";

  return (ContentType::HTML, response);
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Subspace {
    buffer: String
}
use rocket::Data;
use rocket::outcome::Outcome;
use rocket::data::FromData;
/*
impl FromData<'_> for Subspace {
  type Error = String;

  fn from_data(req: &Request, data: Data) -> Outcome<Self, String, String> {
      let mut result = String::new();
      let mut stream = data.open(1.mebibytes());
      stream.read_to_string(&mut result);

      let mut output: Subspace = Default::default();
      output.buffer = result;
      Success(output)
  }
}
*/
#[post("/api/<world>/<coordinate>")]
//fn set(world: &str, coordinate: &str, scroll: Form<PhextScroll>) -> (ContentType, String) {
fn set(world: &str, coordinate: &str) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";
  let message = "TBD"; // scroll;
  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;
  let _result = file.expect(&required).write_all(message.as_bytes());

  return (ContentType::HTML, "Wrote ".to_owned() + &filename);
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