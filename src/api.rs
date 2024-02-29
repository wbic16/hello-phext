#[macro_use] extern crate rocket;

#[get("/<library>.<shelf>.<series>/<collection>.<volume>.<book>/<chapter>.<section>.<scroll>")]
fn phext(library: u8, shelf: u8, series: u8, collection: u8, volume: u8, book: u8, chapter: u8, section: u8, scroll: u8) -> String {
    format!("👋 from {}.{}.{}/{}.{}.{}/{}.{}.{}", library, shelf, series, collection, volume, book, chapter, section, scroll);
}

#[get("/")]
fn hello()
{
    format!("Welcome to phext");
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/api", routes![phext])
}
