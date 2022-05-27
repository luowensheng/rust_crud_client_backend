use super::types::{Post, Database, Posts};
use lazy_static::lazy_static;

lazy_static! {
    static ref DATABASE: Database = Database::new(); 
}

pub fn add(text: String)->(bool, usize){
   DATABASE.insert(text)
}

pub fn get_one(id: usize)->Post{
    DATABASE.get_one(id)
}

pub fn get_all()->Posts{
    DATABASE.get_all()
}

pub fn update(id:usize, text: String)->bool{
    DATABASE.update(id, text)
}

pub fn delete(id:usize)->bool{
    DATABASE.delete(id)
}