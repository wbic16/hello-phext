/// ----------------------------------------------------------------------------------------------------------
/// Phext API Hosting
///
/// The hello-phext repository provides API access to Phext. Refer to README.md for a list of routes.
/// ----------------------------------------------------------------------------------------------------------
#[macro_use] extern crate rocket;
mod phext;
mod phext_test;
use std::{fs::{self, File}, io::Write};
use rocket::Request;
use rocket::http::Status;
use rocket::http::ContentType;
use rocket::form::Form;

/// ----------------------------------------------------------------------------------------------------------
/// @struct Subspace
///
/// Interface class for passing phext data from Rocket into our API endpoints
/// ----------------------------------------------------------------------------------------------------------
#[derive(Default, Debug, PartialEq, Eq, FromForm, Responder)]
struct Subspace {
    content: String,
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn css_rules
///
/// Generates CSS styles for injection within a CSS block of text
/// ----------------------------------------------------------------------------------------------------------
fn css_rules() -> String {
  return "
body {
  background-color: #232323;
  color: #efefff;
  font-family: sans-serif;
  margin: 0 auto;
  width: 90%;
  padding: 10px;
  border: 4px solid grey;
}

textarea {
  background-color: #47579a;
  color: #fefefe;
  font-weight: bold;
  border: 1px solid white;
  border-radius: 3px;
  font-size: 1.4em;
  margin: 10px;
}
input {
  margin: 10px;
  width: 100px;
  padding: 8px;
  border-radius: 3px;
  border: 2px solid white;
  background-color: #47579a;
  color: #fefefe;
  font-weight: bold;
}
input:hover {
  background-color: #9496a7;
  cursor: pointer;
}

input.text {
  border: 1px solid grey;
  width: 150px;
}
input.text:hover {
  cursor: auto;
}

a, a:visited {
  color: #d0d0ff;
  text-decoration: none;
}
a:hover, a:visited:hover {
  color: #ffffff;
  text-decoration: underline;
}

.navmap {
  width: 450px;
  float: left;
  font-family: consolas, monospace;
}
.navmap ul li {
}
.actions {
  position: absolute;
  top: 10px;
  right: 200px;
}
".to_string();
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn css_styling
///
/// Generates an inline HTML/CSS styling block with our preferred styles set.
/// ----------------------------------------------------------------------------------------------------------
fn css_styling() -> String {
  return "<style type=\"text/css\" media=\"all\">".to_owned() + &css_rules() + "</style>";
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn more_cowbell
///
/// ensures that the cowbell character is supported
/// ----------------------------------------------------------------------------------------------------------
#[get("/api/v1/cowbell")]
fn more_cowbell() -> (ContentType, String)
{
  let response = "<html><head><title>More Cowbell</title></head><body>Cowbell: \x07 (Hex = 0x07)</body></html>";
  let passed = phext::check_for_cowbell(response);
  if passed {
    return (ContentType::HTML, response.to_string());
  }

  return (ContentType::HTML, "No cowbell!?".to_string());
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn ignore_warnings
///
/// temporary placeholder for phext methods that only have test coverage so far
/// refer to `cargo test` output for more detail
/// ----------------------------------------------------------------------------------------------------------
#[get("/api/<world>/catchall")]
fn ignore_warnings(world: &str) -> (ContentType, String) {
  let filename: String = world.to_owned() + ".phext";
  let message = "Unable to find ".to_owned() + world;
  let buffer:String = fs::read_to_string(filename).expect(&message);
  let left = buffer.as_str();
  let right = buffer.as_str();
  phext::subtract(left, right);
  let coord = phext::to_coordinate("1.1.1/1.1.1/1.1.1");
  phext::swap(coord, left, right);
  phext::merge(left, right);
  let range = phext::Range { start: phext::to_coordinate("1.1.1/1.1.1/1.1.1"), end: phext::to_coordinate("1.1.1/1.1.1/1.1.2")};
  phext::range_replace(left, range, "test");

  return index(world, "1.1.1/1.1.1/1.1.1");
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn save (index)
/// 
/// This GET masquerades as a call to index, because users are likely to edit a save url to open a new scroll
/// ----------------------------------------------------------------------------------------------------------
#[get("/api/v1/save/<world>/<coordinate>")]
fn save_index(world: &str, coordinate: &str) -> (ContentType, String) {
  return index(world, coordinate);
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn index
///
/// Provides our primary API endpoint for querying phext documents
///
/// @param world       the phext document to load (with the '.phext' extension)
/// @param coordinate  the coordinate to render within `world`
/// ----------------------------------------------------------------------------------------------------------
#[get("/api/v1/index/<world>/<coordinate>")]
fn index(world: &str, coordinate: &str) -> (ContentType, String) {
  let filename: String = world.to_owned() + ".phext";
  let message = "Unable to find ".to_owned() + world;
  let buffer:String = fs::read_to_string(filename).expect(&message);
  let size = buffer.len();
  let scroll = phext::locate(&buffer, coordinate);
  let navmap = phext::navmap(&format!("/api/v1/index/{}/", world), buffer.as_str());

  let coord = coordinate.replace(';', "/");
  let response = "
<html>
  <head>
    <title>Phext API Testing</title>".to_owned() +
    css_styling().as_str() + "
  <script type=\"text/javascript\">
  function dgid(id) {
    return document.getElementById(id);
  }
  function load_event() {
    var se = dgid('scroll_editor');
    if (se) {
      se.focus();
    }
  }
  function expand_scroll() {
    var se = dgid('scroll_editor');
    if (se) {
      var es = dgid('expand_subspace');
      if (es) {
        es.value = se.value;
      }
    }
  }
  function contract_phext() {
    var se = dgid('scroll_editor');
    if (se) {
      var cs = dgid('contract_subspace');
      if (cs) {
        cs.value = se.value;
      }
    }
  }
  function insert_phext() {
    var se = dgid('scroll_editor');
    var iss = dgid('insert_scroll_subspace');
    if (iss) {
      iss.value = se.value;
    }
  }
  function update_phext() {
    var se = dgid('scroll_editor');
    var uss = dgid('update_scroll_subspace');
    if (uss) {
      uss.value = se.value;
    }
  }
  function insert_phext() {
    var se = dgid('scroll_editor');
    var ips = dgid('insert_phext_subspace');
    if (ips) {
      ips.value = se.value;
    }
  }
  function update_phext() {
    var se = dgid('scroll_editor');
    var ups = dgid('update_phext_subspace');
    if (ups) {
      ups.value = se.value;
    }
  }
  function open() {
    var pc = dgid('phext_coordinate');
    if (pc) {
      var coordinate = pc.value.replace('/', ';');
      alert('opening " + &world + " x ' + coordinate);
      //window.open(\"/api/v1/index/" + &world + "/\" + coordinate);
    }
  }
  </script>
  </head>
  <body onLoad=\"load_event();\">
    <div class='navmap'>Phext Viewer<br />" + &world + " (" + &size.to_string() + " bytes):<br />
    Scrolls: " + &navmap + "</div>
    <div class='content'>
      <form method='POST' action='/api/v1/save/" + &world + "/" + coordinate + "'>
        Phext Coordinate: <input class='text' type='text' name='coordinate' id='phext_coordinate' value='" + &coord + "' />
        <input type='submit' value='Save' />
        <input type='button' value='Open' onclick='open();' />
        <input type='hidden' name='world' value='" + &world + "' />
        <br />
        <textarea id='scroll_editor' rows='50' cols='160' name='content'>" + &scroll + "</textarea>
      </form>

      <div class='actions'>
        <form method='POST' action='/api/v1/expand/" + &world + "'>
          <input type='hidden' name='content' id='expand_subspace' value='' />
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Expand' onclick='expand_phext();' />
        </form>

        <form method='POST' action='/api/v1/contract/" + &world + "'>
          <input type='hidden' name='content' id='contract_subspace' value='' />
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Contract' onclick='contract_phext();' />
        </form>

        <form method='POST' action='/api/v1/insert/" + &world + "/" + &coordinate + "'>
          <input type='hidden' name='content' id='insert_scroll_subspace' value='' />
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Insert Scroll' onclick='insert_scroll();' />
        </form>

        <form method='POST' action='/api/v1/update/" + &world + "/" + &coordinate + "'>
          <input type='hidden' name='content' id='update_scroll_subspace' value='' />
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Update Scroll' onclick='update_scroll();' />
        </form>

        <form method='POST' action='/api/v1/delete/" + &world + "/" + &coordinate + "'>
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Delete Scroll' />
        </form>

        <form method='POST' action='/api/v1/insert/" + &world + "'>
          <input type='hidden' name='content' id='insert_phext_subspace' value='' />
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Insert Phext' onclick='insert_phext();' />
        </form>

        <form method='POST' action='/api/v1/update/" + &world + "'>
          <input type='hidden' name='content' id='update_phext_subspace' value='' />
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Update Phext' onclick='update_phext();' />
        </form>

        <form method='POST' action='/api/v1/delete/" + &world + "'>
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Delete Phext' />
        </form>
      </div>

    </div>
  </body>
</html>";

  return (ContentType::HTML, response);
}

#[get("/favicon.ico")]
fn favorite_icon() -> (ContentType, Vec<u8>) {
  let favicon = include_bytes!("favicon.ico");
  return (ContentType::Icon, favicon.to_vec());
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn select_scroll
///
/// retrieves just the raw scroll for a given phext coordinate
/// ----------------------------------------------------------------------------------------------------------
#[get("/api/v1/select/<world>/<coordinate>")]
fn select_scroll(world: &str, coordinate: &str) -> (ContentType, String) {
  let filename: String = world.to_owned() + ".phext";
  let message = "Unable to find ".to_owned() + world;
  let buffer:String = fs::read_to_string(filename).expect(&message);
  let scroll = phext::locate(&buffer, coordinate);

  return (ContentType::Text, scroll);
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn select_phext
///
/// retrieves the entire phext (copy/paste at scale!)
/// ----------------------------------------------------------------------------------------------------------
#[get("/api/v1/select/<world>")]
fn select_phext(world: &str) -> (ContentType, String) {
  let filename: String = world.to_owned() + ".phext";
  let message = "Unable to find ".to_owned() + world;
  let buffer:String = fs::read_to_string(filename).expect(&message);

  return (ContentType::Text, buffer);
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn insert_scroll
///
/// inserts a new scroll (or appends to the existing scroll) at the given coordinate
/// ----------------------------------------------------------------------------------------------------------
#[post("/api/v1/insert/<world>/<coordinate>", data="<scroll>")]
fn insert_scroll(world: &str, coordinate: &str, scroll: Form<Subspace>) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";
  
  let prior = fs::read_to_string(filename.clone()).expect("Unable to open world");
  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;

  let message = phext::insert(prior.as_str(), phext::to_coordinate(coordinate), scroll.content.as_str());
  let _result = file.expect(&required).write_all(message.as_bytes());

  return (ContentType::Text, "OK".to_string());
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn insert_phext
///
/// inserts a new scroll (or appends to the existing scroll) at the given coordinate
/// ----------------------------------------------------------------------------------------------------------
#[post("/api/v1/insert/<world>", data="<phext>")]
fn insert_phext(world: &str, phext: Form<Subspace>) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";
  
  let prior = fs::read_to_string(filename.clone()).expect("Unable to open world");
  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;  
  let message = prior + &phext.content;
  let _result = file.expect(&required).write_all(message.as_bytes());

  return (ContentType::Text, "OK".to_string());
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn update_scroll
/// 
/// replaces the contents of the specified scroll
/// ----------------------------------------------------------------------------------------------------------
#[post("/api/v1/update/<world>/<coordinate>", data="<scroll>")]
fn update_scroll(world: &str, coordinate: &str, scroll: Form<Subspace>) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";
  
  let prior = fs::read_to_string(filename.clone()).expect("Unable to open world");
  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;

  let message = phext::replace(prior.as_str(), phext::to_coordinate(coordinate), scroll.content.as_str());
  let _result = file.expect(&required).write_all(message.as_bytes());

  return (ContentType::Text, "OK".to_string());
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn update_phext
/// 
/// replaces the contents of the specified scroll
/// ----------------------------------------------------------------------------------------------------------
#[post("/api/v1/update/<world>", data="<phext>")]
fn update_phext(world: &str, phext: Form<Subspace>) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";
  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;
  let _result = file.expect(&required).write_all(phext.content.as_bytes());

  return (ContentType::Text, "OK".to_string());
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn delete_scroll
/// 
/// zeroes the length of the given scroll
/// ----------------------------------------------------------------------------------------------------------
#[post("/api/v1/delete/<world>/<coordinate>")]
fn delete_scroll(world: &str, coordinate: &str) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";
  
  let prior = fs::read_to_string(filename.clone()).expect("Unable to open world");
  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;

  let coord = phext::to_coordinate(coordinate);
  let message = phext::remove(prior.as_str(), coord);
  let _result = file.expect(&required).write_all(message.as_bytes());

  return (ContentType::Text, "OK".to_string());
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn delete_phext
/// 
/// zeroes the length of the given scroll
/// ----------------------------------------------------------------------------------------------------------
#[post("/api/v1/delete/<world>")]
fn delete_phext(world: &str) -> (ContentType, String) {
  let empty:Subspace = Subspace{ content: "".to_string() };
  let nothing: Form<Subspace> = Form::from(empty);
  return update_phext(world, nothing);
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn save
///
/// Provides a POST API endpoint for accepting a phext scroll oriented at a specific coordinate
///
/// @param world       the phext document to save (not including the .phext extension)
/// @param coordinate  the phext coordinates within `world` to edit
/// @param scroll      content to replace at the given coordinates
/// ----------------------------------------------------------------------------------------------------------
#[post("/api/v1/save/<world>/<coordinate>", data="<scroll>")]
fn save(world: &str, coordinate: &str, scroll: Form<Subspace>) -> (ContentType, String) {
  let _result = update_scroll(world, coordinate, scroll);

  return index(world, coordinate);
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn normalize
///
/// Provides phext normalization - trimming unused pockets of subspace from an input phext.
/// WARNING: Overwrites the entire contents of `world`!
/// ----------------------------------------------------------------------------------------------------------
#[post("/api/v1/normalize/<world>", data="<scroll>")]
fn normalize(world: &str, scroll: Form<Subspace>) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";

  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;

  let message = phext::normalize(scroll.content.as_str());
  let _result = file.expect(&required).write_all(message.as_bytes());

  return index(world, "1.1.1/1.1.1/1.1.1");
}


/// ----------------------------------------------------------------------------------------------------------
/// @fn contract
///
/// Provides phext contraction - transforms all dimension breaks down by 1
/// WARNING: Overwrites the entire contents of `world`!
/// ----------------------------------------------------------------------------------------------------------
#[post("/api/v1/contract/<world>", data="<scroll>")]
fn contract(world: &str, scroll: Form<Subspace>) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";

  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;

  let message = phext::contract(scroll.content.as_str());
  let _result = file.expect(&required).write_all(message.as_bytes());

  return index(world, "1.1.1/1.1.1/1.1.1");
}


/// ----------------------------------------------------------------------------------------------------------
/// @fn expand
///
/// Provides phext expansion - transforms all dimension breaks up by 1
/// WARNING: Overwrites the entire contents of `world`!
/// ----------------------------------------------------------------------------------------------------------
#[post("/api/v1/expand/<world>", data="<scroll>")]
fn expand(world: &str, scroll: Form<Subspace>) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";

  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;

  let message = phext::expand(scroll.content.as_str());
  let _result = file.expect(&required).write_all(message.as_bytes());

  return index(world, "1.1.1/1.1.1/1.1.1");
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn not_found
///
/// Provides a specific error message for unrecognized URLs, instructing the user to reach out to us on twitter.
/// ----------------------------------------------------------------------------------------------------------
#[catch(404)]
fn not_found(req: &Request) -> String {
  return format!("Unable to locate '{}'. Reach out to @wbic16 on twitter.", req.uri());
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn default
///
/// Handles generic errors - informs the user that an unexpected error has occurred
/// ----------------------------------------------------------------------------------------------------------
#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    return format!("{} ({})", status, req.uri());
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn rocket
///
/// Builds a rocket instance, registers default and 404 pages, and mounts our GET/POST endpoints
/// ----------------------------------------------------------------------------------------------------------
#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found, default])
        .mount("/", routes![select_scroll, select_phext,
                            insert_scroll, insert_phext,
                            update_scroll, update_phext,
                            delete_scroll, delete_phext,
                            index, save, normalize, expand, contract,
                            save_index,
                            favorite_icon,
                            more_cowbell,
                            ignore_warnings])
}