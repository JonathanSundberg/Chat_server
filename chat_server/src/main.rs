#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
use serde::{Deserialize, Serialize, de::Error};
use serde_json;
use std::{collections::HashMap, collections::HashSet, fs::File, path::PathBuf, result};
use rocket_contrib::json::{self, Json};
use std::io::BufReader;

#[macro_use] extern crate rocket;


#[derive(Deserialize, Serialize, Debug)]
struct Message{
    message: String,
    user: String,
    complete: bool
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
struct User{
    user_name: String,
    password: String,
    email: String
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct UserDatabase{
    users: HashMap<String, User>,
    emails: HashSet<String>
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


fn _check_if_username_exists(database: &UserDatabase, user: &User) -> bool{
    database.users.contains_key(&user.user_name)
}
fn _check_if_email_exists(database: &UserDatabase, user: &User) -> bool{
    database.emails.contains(&user.email)
}
fn _get_file_if_exists_else_create_empty(filepath: PathBuf) -> File{
    if filepath.exists(){
        return File::open(filepath).unwrap()
    }
    else{
        return File::create(filepath).unwrap()
    };
}

fn _parse_json_file(json_file: File) -> Option<UserDatabase>{
    let reader = BufReader::new(json_file);
    let json_database: Result<UserDatabase, serde_json::Error> = serde_json::from_reader(reader);
    match json_database{
        Ok(content) => Some(content),
        Err(_) => None
    }
}

fn mounts() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![update_messages])
        .mount("/", routes![received_message])
        .mount("/", routes![register_user])
}

fn initialze(){
    
}

fn main() {
    initialze();
    //mounts().launch();
}


// use cargo test -- --nocapture to get output
#[cfg(test)]
mod tests{
    use rocket::http::{ContentType};
    use rocket_contrib::json;
    use serde::__private::de::Content;

    use super::*;
    use super::User;
    use super::UserDatabase;
    use super::Message;
    use rocket::local::Client;
    use rocket::http::Status;
    use std::fs::File;
    use std::io::BufReader;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::path::Path;
    use std::str::FromStr;



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
    fn register_user() {

        let temp_user_1 = User{
            user_name: "my_user".to_string(),
            password: "testing_my_password".to_string(),
            email: "test_email@trying.com".to_string()
        };
        let temp_user_2 = User{
            user_name: "second_user".to_string(),
            password: "Another_password".to_string(),
            email: "another_email@trying.com".to_string()
        };
        
        let filepath = PathBuf::from_str("Users.json").unwrap();
        let database_file = _get_file_if_exists_else_create_empty(filepath);
        let json_database = _parse_json_file(database_file);

        let mut json_content = match json_database{
            Some(content) => content,
            None => {
                UserDatabase::default()
            }
        };


        println!("Database content: {:?}", json_content);


        if _check_if_username_exists(&json_content, &temp_user_2){
            println!("Username exists");
            //return;
        }
        if _check_if_email_exists(&json_content, &temp_user_2){
            println!("Email exists");
            return;
        }

        json_content.users.insert(temp_user_2.user_name.clone(), temp_user_2.clone());
        json_content.emails.insert(temp_user_2.email.clone());
        println!("{:?}", &json_content);

        let new_file = File::create("Users.json").unwrap();

        serde_json::to_writer_pretty(new_file, &json_content).expect("Could not write to the Users.json file");
    }

    #[test]
    fn check_if_user_exists(){

    }

}