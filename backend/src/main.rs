#[macro_use] extern crate rocket;
#[macro_use] mod utils;
mod api;
pub use api::text::index::{get_all, get_by_id, update, delete, create};

// use rocket::response::*;

#[launch]
fn rocket() -> _ {
    

    rocket::build()
                 .mount("/api/v1/text", routes![get_all, get_by_id, update, delete, create])
}

