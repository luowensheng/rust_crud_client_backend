use std::{collections::HashMap, sync::Mutex, ops::{DerefMut, Deref}};

use rocket::{Data, data::{FromData, self, ByteUnit}, http::{ContentType, Status}, request, Request, Error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    text: String,
    id: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostInput {
    pub text: String,
}

#[derive(Debug)]
pub enum PostFormError {
    ParseError
}


#[rocket::async_trait]
impl<'r> FromData<'r> for PostInput {
    type Error = PostFormError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let person_ct = ContentType::new("application", "json");
        if req.content_type() != Some(&person_ct) {
            return Forward(data);
        }

        let limit = req.limits().get("text").unwrap_or(ByteUnit::Byte(256));

        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, PostFormError::ParseError)),
            Err(_) => return Failure((Status::InternalServerError, PostFormError::ParseError)),
        };

        let string: &str = request::local_cache!(req, string);

        let (_, content) = match string.find(':') {
            Some(i) => (&string[..i], &string[(i + 2)..string.len()-2]),
            None => return Failure((Status::UnprocessableEntity, PostFormError::ParseError)),
        };

        println!("\n\ncontent=[{}]\n\n", content);
        
        Success(PostInput { text:content.to_string() })
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
pub type Posts = Vec<Post>;
pub trait JsonAble {
    fn to_json(&self)->String;   
}


impl JsonAble for Posts {
    fn to_json(&self)->String {
        serde_json::to_string(&self).unwrap()
    }
}


impl Post {
    pub fn new(text: String, id: usize)->Post{
         Post{text,  id}
    }

    pub fn empty()->Post{
          Post { text: "[empty]".to_string(), id:0} 
    }
}

impl JsonAble for Post {
    fn to_json(&self)->String{
        serde_json::to_string(self).unwrap()
    }
}


pub struct  Database {
    data:Mutex<HashMap<usize, Post>>
}

impl Deref for Database {
    type Target = Mutex<HashMap<usize, Post>>;

    fn deref(&self) -> &Self::Target {
         &self.data
        
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub trait DatabaseAble {
    fn insert(&self, text: String)->(bool, usize);
    fn insert_by_id(&self, text: String, id: usize)->bool;
    fn get_one(&self, id: usize)->Post;
    fn update(&self, id: usize, text: String)->bool;
    fn delete(&self, id: usize)->bool;
    fn get_some(&self, ids: Vec<usize>)->Posts;
    fn get_all(&self)->Posts;
}


impl Database {
    pub fn new()->Database{
        Database { data: Mutex::new(HashMap::new()) }

    }
}

impl DatabaseAble for Database {

     fn insert(&self, text: String)->(bool, usize){
            let id = self.deref().lock().unwrap().len();
            (self.insert_by_id(text, id), id)
            
    }

     fn insert_by_id(&self, text: String, id: usize)->bool {
        
        let mut data = self.deref().lock().unwrap();
        let item = data.get(&id);

        match item {
            Some(_) => false,
            None => {
                data.insert(id, Post::new(text, id));
                true
            },
        }
        
    }

     fn get_one(&self, id: usize)->Post{

        match self.deref().lock().unwrap().get(&id) {
            Some(post) => post.clone(),
            None => Post::empty()
        }
        
    }

     fn update(&self, id: usize, text: String)->bool{

        let delete_successful = self.delete(id);
        if delete_successful {
            self.insert_by_id(text, id)
        } else {
            false
        }

    }

     fn delete(&self, id: usize)->bool{
        let id_exists = self.get_one(id).id!=0;
        if id_exists {
            self.deref().lock().unwrap().remove(&id);  
            return true;
        } else {
            false
        }
    }

     fn get_some(&self, ids: Vec<usize>)->Posts{
        let mut posts: Posts = vec![];
        for id in ids {
            let post = self.get_one(id);
            if post.id != 0 {
                posts.push(post.to_owned());
            }
        }
        return posts;
    }

     fn get_all(&self)->Posts{
        let mut posts: Posts = vec![];
 
        for (_, post) in self.deref().lock().unwrap().iter(){
                   posts.push(post.clone());
        }
        return posts;
    }

}




