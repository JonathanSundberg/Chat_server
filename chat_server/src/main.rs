#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
use serde::{Deserialize, Serialize};
use serde_json;
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

#[derive(Deserialize, Serialize, Debug)]
struct UserDatabase{
    Users: Vec<User>
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


fn _check_if_username_exists(database: &UserDatabase, username: &String) -> bool{
    for user in database.Users.iter(){

        if user.user_name == username.clone(){
            return true
        }
    }
    false
}
fn _check_if_email_exists(database: &UserDatabase, email: &String) -> bool{
    for user in database.Users.iter(){

        if user.email == email.clone(){
            return true
        }
    }
    false
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

    use super::mounts;
    use super::User;
    use super::UserDatabase;
    use super::Message;
    use rocket::local::Client;
    use rocket::http::Status;
    use std::fs::File;
    use std::io::BufReader;


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

        let temp_user = User{
            user_name: "my_user".to_string(),
            password: "testing_my_password".to_string(),
            email: "test_email@trying.com".to_string()
            
        };

        let database_file = File::create("Users.json").unwrap();
        let reader = BufReader::new(&database_file);

        let json_database: UserDatabase = match serde_json::from_reader(reader){
            Ok(content) => content,
            Err(e) => {
                println!("Error when reading Users.json, creating empty database..: {}", e);
                let empty_vec = vec![];
                UserDatabase{
                    Users: empty_vec
                }
            }
        };

        serde_json::to_writer_pretty(database_file, &temp_user).expect("Could not write to the Users.json file");
    }
    #[test]
    fn check_if_user_exists(){

    }

}