#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket::{Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", FileServer::from("static"))
}