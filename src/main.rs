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
  width: 60%;
}
input {
  margin: 10px;
  width: 120px;
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
/// @fn liquid
///
/// provides the liquid data visualizer
/// ----------------------------------------------------------------------------------------------------------
#[get("/api/v1/liquid/<world>/<coordinate>")]
fn liquid(world: &str, coordinate: &str) -> (ContentType, String)
{
  let css = "
  body {
    background: #101419;
    color: white;
    font-family: sans-serif;
  }
  a, a:visited {
    color: white;
    font-size: 1.6em;
  }
  a.small, a.small:visited {
    font-size: 1em;
  }
  .number {
    font-weight: bold;
    color: #8f8fd7;
  }
  .coordinates {
    font-style: italic;
    color: #8fef62;
  }
  input {
    border: 2px solid white;
    border-radius: 1px;
    padding: 4px;
    margin: 1px;
    margin-bottom: 4px;
  }
  #city {
    position: relative;
    top: 50px;
    left: 10%;
    width: 90%;
  }
  #present {
    position: absolute;
    z-index: 4;
    top: 100px;
    left: -1000px;
    width: 800px;
    height: 400px;
    font-size: 2em;
    background: white;
    border: 16px solid orange;
    color: black;
    padding: 20px;
  }
  .summary {
    position: absolute;
    top: -30px;
  }
  .outer {
    background-color: #1B4079;
    border-radius: 20px;
    color: white;
    font-weight: bold;
    font-size: 1em;
    z-index: 0;
  }
  .room {
    width: 60px;
    height: 48px;
    color: #202030;
    background-color: #B4C5E4;
    border-radius: 3px;
    z-index: 1;
  }
  .outer,
  .room {
    position: absolute;
    border: 2px solid grey;
    text-align: center;
    vertical-align: center;
    margin-bottom: 4px;
    padding-top: 2px;
    transition: all 0.4s;
  }
  .room:hover {
    cursor: pointer;
    background: whitesmoke;
    color: black;
    scale: 2;
    width: 80px;
    height: 40px;
    padding-top: 10px;
    z-index: 3;
  }
  .outer {
    width: 670px;
    height: 540px;
  }
  #presentCloser {
    text-decoration: underline; cursor: pointer;
  }
  #presentCloser:hover {
    background-color: grey;
  }
  ";
  let js = "
  <script type=\"text/JavaScript\">

  function dgid(id) {
    return document.getElementById(id);
  }
  
  var MAJOR_WIDTH = 720;
  var MAJOR_HEIGHT = 600;
  
  function hide() {
    var present = dgid(\"present\");
    present.style.left = \"-2500px\";
  }
  
  function show(cellColumn, cellRow, column, row, chapter, section, scroll) {
    squeeze(cellColumn, cellRow, column, row);    
  }
  
  function randomInteger(limit) {
    return Math.floor(Math.random() * (limit + 1));
  }
    
  function setupCity() {
    var city = dgid(\"city\");
    var output = \"\";
    var section = 1;
    var scroll = 1;
    var chapter = 1;
    var total = 0;
    var left = 0;
    var top = 0;
    var ileft = 0;
    var itop = 0;
    for (var j = 1; j <= 11; ++j) {
      for (var i = 1; i <= 11; ++i) {
        left = (MAJOR_WIDTH * (i-1));
        top = (MAJOR_HEIGHT * (j-1));
        output += \"<div id='outer_\" + i + \"_\" + j + \"' class='outer' style='top: \" + top + \"px; left: \" + left + \"px;'>\" + chapter + \".\" + section + \".\" + scroll + \"\\n\";
        for (var y = 1; y <= 9; ++y) {
          for (var x = 1; x <= 9; ++x) {
            ileft = 64 * x;
            itop = 48 * y;
            output += \"<div id='inner_\" + i + \"_\" + j + \"_\" + x + \"_\" + y + \"' class='room' style='position: absolute; top: \" + itop + \"px; left: \" + ileft + \"px;' onclick='show(\" + i + \", \" + j + \", \" + x + \",\" + y + \",\" + chapter + \",\" + section + \",\" + scroll + \");'>\" + scroll + \"</div>\\n\";
            scroll += 1;
            total += 1;
            if (scroll > 99) {
              scroll = 1;
              section += 1;
            }
            if (section > 99) {
              scroll = 1;
              section = 1;
              chapter += 1;
            }
          }
        }
        output += \"</div>\\n\";
      }
    }
  
    var summary = \"<div class='summary'>Rooms on this Block (1.1.1/1.1.1/1.*.*): \" + total + \" (\" + Math.round(100*2*total/1024)/100 + \" MB)</div><br />\\n\";
    city.innerHTML = summary + output;
  }
  
  function getOuter(w, x) {
    return dgid(\"outer_\" + w + \"_\" + x);
  }
  
  function getInner(w, x, y, z) {
    return dgid(\"inner_\" + w + \"_\" + x + \"_\" + y + \"_\" + z);
  }

  function phextCoordinateToGridCoordinate(coord) {
    var parts = coord.split('/');
    var z = parts[0]; var y = parts[1]; var x = parts[2];
    var zp = z.split('.'); var yp = y.split('.'); var xp = x.split('.');
    var lb = zp[0]; var sf = zp[1]; var sr = zp[2];
    var cn = yp[0]; var vm = yp[1]; var bk = yp[2];
    var ch = xp[0]; var sn = xp[1]; var sc = xp[2];
    var position = parseInt(99*(sn-1)) + parseInt(sc) - 1;
    var blocks = Math.floor(position/81); var ox = Math.floor(blocks/11)+1; var ow = blocks%11+1;
    var remainder = position%81; var oz = Math.floor(remainder/9)+1; var oy = remainder%9+1;
    return ow + \"_\" + ox + \"_\" + oy + \"_\" + oz;
  }

  function getPhextCell(coord) {
    var id = 'inner_' + phextCoordinateToGridCoordinate(coord);
    var handle = dgid(id);
    if (handle) {
      return handle;
    }

    return false;
  }
  
  function squeeze(w, x, y, z) {
    var cell = getOuter(w, x);
    var inner = getInner(w, x, y, z);
    if (cell) {
    }
    if (inner && inner.style.scale.length == 0) {
      inner.style.scale = \"4.0\";
      inner.style.zIndex = \"3\";
    } else if (inner) {
      inner.style.scale = \"\";
      inner.style.zIndex = \"3\";
    }
    for (var i = 1; i <= 9; ++i) {
      for (var j = 1; j <= 9; ++j) {
        var adjust = getInner(w, x, i, j);      
        if (adjust && (adjust != inner) && adjust.style.scale.length == 0) {
          adjust.style.scale = \"\";
        }
      }
    }
  }
  
  function cleanup(w, x) {
    for (var y = 1; y <= 9; ++y) {
      for (var z = 1; z <= 9; ++z) {
        var cell = getInner(w, x, y, z);
        if (cell) {
          cell.style.scale = \"\";
          cell.style.zIndex = \"\";
        }      
      }
    }
  
    var cell = getOuter(w, x);
    if (cell) {
      cell.style.scale = \"\";
    }
  }
  </script>
  ";
  let response = "<html>
<head>
<title>Liquid Metal</title>
<style type='text/css' media='all'>".to_string() +
css + "
</style>" +
js + "
</head>
<body onload=\"setupCity();\">
  <a href=\"https://phext.io/white-rabbit.html?m=unlocked\">return to game</a>
<div id=\"city\"></div>
<div id=\"present\"></div>
</body></html>";

  return (ContentType::HTML, response);
}


/// ----------------------------------------------------------------------------------------------------------
/// @fn subtract
///
/// removes scrolls that have content in both archives from the first archive
/// ----------------------------------------------------------------------------------------------------------
#[get("/api/v1/subtract/<world>/<other>")]
fn subtract(world: &str, other: &str) -> (ContentType, String)
{
  let filename = world.to_owned() + ".phext";
  let left = fetch_phext_buffer(world);
  let right = fetch_phext_buffer(other);
  let result = phext::subtract(left.as_str(), right.as_str());
  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;
  let _result = file.expect(&required).write_all(result.as_bytes());

  return (ContentType::HTML, "OK".to_string());
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn merge
///
/// zipper merge for two phexts into one
/// ----------------------------------------------------------------------------------------------------------
#[get("/api/v1/merge/<world>/<mother>/<father>")]
fn merge(world: &str, mother: &str, father: &str) -> (ContentType, String)
{
  let filename = world.to_owned() + ".phext";
  let left = fetch_phext_buffer(mother);
  let right = fetch_phext_buffer(father);
  let result = phext::merge(left.as_str(), right.as_str());
  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;
  let _result = file.expect(&required).write_all(result.as_bytes());

  return index(world, "1.1.1/1.1.1/1.1.1");
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn fetch_phext_buffer
///
/// Retrieves the content from the .phext archive specified by `world`
/// ----------------------------------------------------------------------------------------------------------
fn fetch_phext_buffer(world: &str) -> String {
  let filename: String = world.to_owned() + ".phext";
  let message = "Unable to find ".to_owned() + world;
  let buffer:String = fs::read_to_string(filename).expect(&message);
  return buffer;
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
  let buffer = fetch_phext_buffer(world);
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
  function subtract() {
    var sf = dgid('subtract_form');
    if (sf.action.endsWith('__other__')) {
      var phext = prompt(\"Which phext to mask with?\");
      sf.action = sf.action.replace('__other__', phext);
    }
  }
  function merge() {
    var mf = dgid('merge_form');
    if (mf.action.endsWith('__mother__/__father__')) {
      var mother = dgid('mother').value;
      var father = dgid('father').value;
      mf.action = mf.action.replace('__mother__', mother);
      mf.action = mf.action.replace('__father__', father);
    }
  }
  function replace() {
    var rf = dgid('replace_form');
    var rc = dgid('replace_content');
    var se = dgid('scroll_editor');
    if (rf.action.endsWith('__start__/__end__')) {
      var start = dgid('start').value;
      var end = dgid('end').value;
      rf.action = rf.action.replace('__start__', start);
      rf.action = rf.action.replace('__end__', end);
      rc.value = se.value;
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
        <textarea id='scroll_editor' rows='50' name='content'>" + &scroll + "</textarea>
      </form>

      <div class='actions'>
        <h2>Scroll Operations</h2>

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

        <h2>Phext Operations</h2>
        <hr />

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

        <form method='POST' action='/api/v1/insert/" + &world + "'>
          <input type='hidden' name='content' id='insert_phext_subspace' value='' />
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Insert' onclick='insert_phext();' />
        </form>

        <form method='POST' action='/api/v1/update/" + &world + "'>
          <input type='hidden' name='content' id='update_phext_subspace' value='' />
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Update' onclick='update_phext();' />
        </form>

        <form method='POST' action='/api/v1/delete/" + &world + "'>
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Delete' />
        </form>

        <form method='GET' id='subtract_form' action='/api/v1/subtract/" + &world + "/__other__'>
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Subtract' onclick='subtract();' />
        </form>

        <form method='GET' id='merge_form' action='/api/v1/merge/" + &world + "/__mother__/__father__'>
          Mother: <input type='text' id='mother' /><br />
          Father: <input type='text' id='father' /><br />
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Merge' onclick='merge();' />
        </form>

        <form method='POST' id='replace_form' action='/api/v1/replace/" + &world + "/__start__/__end__'>          
          Start Coordinate: <input type='text' id='start' value='1.1.1;1.1.1;1.1.1' /><br />
          End Coordinate: <input type='text' id='end' value='1.1.1;1.1.1;1.1.1' /><br />
          <input type='hidden' name='content' id='replace_content' />
          <input type='hidden' name='redirect' value='yes' />
          <input type='submit' value='Replace' onclick='replace();' />
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
  let buffer = fetch_phext_buffer(world);
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
  let buffer = fetch_phext_buffer(world);
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
  let prior = fetch_phext_buffer(world);
  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;

  let message = phext::insert(prior, phext::to_coordinate(coordinate), scroll.content.as_str());
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
  let prior = fetch_phext_buffer(world);
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
  let prior = fetch_phext_buffer(world);
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
  let prior = fetch_phext_buffer(world);
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
/// @fn range_replace
///
/// Inserts the content of `scroll`, overwriting all content from `start` to `end`
/// ----------------------------------------------------------------------------------------------------------
#[post("/api/v1/replace/<world>/<start>/<end>", data="<scroll>")]
fn range_replace(world: &str, start: &str, end: &str, scroll: Form<Subspace>) -> (ContentType, String) {
  let filename = world.to_owned() + ".phext";
  let range = phext::Range { start: phext::to_coordinate(start), end: phext::to_coordinate(end) };
  let prior = fetch_phext_buffer(world);

  let file = File::create(&filename);
  let required = "Unable to locate ".to_owned() + &filename;

  let message = phext::range_replace(prior.as_str(), range, scroll.content.as_str());
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
                            save_index, subtract, merge, range_replace,
                            favorite_icon, liquid, more_cowbell])
}