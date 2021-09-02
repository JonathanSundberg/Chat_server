#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
use serde::{Deserialize, Serialize};
//use serde_json;
use rocket_contrib::json::Json;
#[macro_use] extern crate rocket;


#[derive(Deserialize, Serialize, Debug)]
struct Message{
    message: String,
    user: String,
    complete: bool
}

#[derive(Deserialize, Serialize, Debug)]
struct User{
    user_name: String,
    password: String,
    email: String
}



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/message/update/<sent_string>")]
fn update_messages(sent_string: &RawStr) -> String{
    //println!("asdasdasdasdasd");
    format!("This is the string:, {}", sent_string.as_str())
}

// curl -X POST -H "Content-Type: application/json" -d @post_json.json http://localhost:8000/message/received  too test
#[post("/message/received", format = "json", data = "<message>")]
fn received_message(message: Json<Message>) -> String {
    println!("message: {}", &message.message);
    format!("We are getting a post request!")
}

#[post("/message/register", format = "json", data = "<_user>")]
fn register_user(_user: Json<User>) -> String {
    println!("We are registering a user!");
    format!("User registered!")
}





fn mounts() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![update_messages])
        .mount("/", routes![received_message])
        .mount("/", routes![register_user])
}

fn main() {
    mounts().launch();
}

// use cargo test -- --nocapture to get output
#[cfg(test)]
mod tests{

    use rocket::http::{ContentType};

    use super::mounts;
    use super::Message;
    use rocket::local::Client;
    use rocket::http::Status;
    use rusqlite;
    use std::collections::HashMap;


    #[test]
    fn update_messages(){
        let client = Client::new(mounts()).expect("valid rocket instance");
        let mut response = client.get("/message/update/Testing_a_string_here").dispatch();
        assert_eq!(response.status(), Status::Ok);
        //assert_eq!(response.body_string(), Some("Hello, world!".into()));

        println!("message string: {}", response.body_string().unwrap());
    }

    #[test]
    fn received_message(){

        let message = Message{
            message: String::from("this is my test string"),
            user: String::from("test user"),
            complete: true
        };
        let body = serde_json::to_string(&message).unwrap();
        
        let client = Client::new(mounts()).expect("valid rocket instance");
        let _result = client.post("/message/received")
        .header(ContentType::JSON)
        .body(&body)        
        .dispatch();

        //println!("response: {}", res);
    }

    #[test]
    fn register_user() -> Result<(), rusqlite::Error>{
        let conn = rusqlite::Connection::open("Users.db")?;
        conn.execute(
            "CREATE TABLE if not exists cat_colors (
                 id integer primary key,
                 name text not null unique
             )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE if not exists cats (
                 id integer primary key,
                 name text not null,
                 color_id integer not null references cat_colors(id)
             )",
            [],
        )?;
    
        Ok(())

    }
}