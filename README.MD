```
use rocket::http::*;

use super::{database, types::{Posts, PostInput, JsonAble}};


#[get("/")]
pub fn get_all() -> (Status, (ContentType, String)) {
    let data:Posts = database::get_all();
    (Status::ImATeapot, (ContentType::JSON, data.to_json()))
}

#[get("/<id>")]
pub fn get_by_id(id:usize) -> (Status, (ContentType, String)) {
    let data = database::get_one(id); 
    (Status::ImATeapot, (ContentType::JSON, data.to_json()))
}

#[post("/", format ="application/json", data = "<new_post>")]
pub fn create(new_post: PostInput) -> (Status, (ContentType, String)) {

    let (_,id) = database::add(new_post.text);
    let data = database::get_one(id);
    (Status::ImATeapot, (ContentType::JSON, data.to_json()))
}

#[put("/<id>", format ="application/json", data = "<new_post>")]
pub fn update(id:usize, new_post: PostInput) -> (Status, (ContentType, String)) {

    database::update(id, new_post.text);
    let data = database::get_one(id);
    (Status::ImATeapot, (ContentType::JSON, data.to_json()))
}

#[delete("/<id>")]
pub fn delete(id:usize) -> (Status, (ContentType, String)) {

    database::delete(id);
    let data = database::get_one(id);
    (Status::ImATeapot, (ContentType::JSON, data.to_json()))
}
```