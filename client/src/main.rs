use std::collections::HashMap;

//https://blog.logrocket.com/making-http-requests-rust-reqwest/
// use futures::TryFutureExt;
use reqwest::{self, Client};

static URL: &str = "http://127.0.0.1:8000/api/v1/text";

async fn get_request(url: &String){
    let result = reqwest::get(url).await
                                                                            .unwrap()
                                                                            .text()
                                                                            .await
                                                                            ;
    println!("[GET] {:?}", result.unwrap())
}

async fn post_request(client: &Client, url: &String, text: &&str){
        let mut map =  HashMap::new();
        map.insert("text", text);

        let result = client.post(url)
        .json(&map)
        .send()
        .await.unwrap()
        .text().await;
        println!("[POST] {:?}", result.unwrap())

}

async fn put_request(client: &Client, url: &String, text: &&str){
    let mut map =  HashMap::new();
    map.insert("text", text);

    let result = client.put(url)
    .json(&map)
    .send()
    .await.unwrap()
    .text().await;
    println!("[PUT] {:?}", result.unwrap())

}

async fn delete_request(client: &Client, url: &String){
    let result = client.delete(url)
    .send()
    .await.unwrap()
    .text().await;
    println!("[DELETE] {:?}", result.unwrap())

}


async fn test_get(){
    for i in 0..50{
        let num_str = (i as u32).to_string();
        let url = URL.to_string()+"/"+&num_str;
        get_request(&url).await;        
    }
}

async fn test_getall(){
      get_request(&URL.to_string()).await;        
}

async fn test_post(){
    let client = reqwest::Client::new();
    for i in 0..500{
        let mut post  = "this is post #".to_string();
        post.push_str((i as u64).to_string().as_str());
        print!("{}", &post);
        post_request(&client, &URL.to_string(), &post.as_str()).await;        
    }
}

async fn test_put(){
    let client = reqwest::Client::new();
    for i in 0..50{
        let num_str = (i as u32).to_string();
        let mut post  = "[updated] this is post #".to_string();
        post.push_str((i as u32).to_string().as_str());
        let url = URL.to_string()+"/"+&num_str;
        put_request(&client, &url, &post.as_str()).await;        
    } 
}
async fn test_delete(){
    let client = reqwest::Client::new();
    for i in 0..50{
        let num_str = (i as u32).to_string();
        let url = URL.to_string()+"/"+&num_str;
        delete_request(&client, &url).await;        
    }
}
#[tokio::main]
async fn main() {

    test_post().await;
    test_get().await;
    test_put().await;
    test_delete().await;
    test_getall().await;

}