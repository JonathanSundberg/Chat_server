#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
use serde::{Deserialize, Serialize};
use serde_json;
use rocket_contrib::json::Json;
#[macro_use] extern crate rocket;


#[derive(Deserialize, Serialize)]
struct Message{
    message: String,
    user: String,
    complete: bool
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

#[post("/message/received", format = "json", data = "<message>")]
fn received_message(message: Json<Message>) -> String {
    println!("asdasdasdasdasd");
    format!("We are getting a post request!")

}

#[post("/message/temp")]
fn temp() {
    println!("This is the temp method");

}


fn mounts() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![update_messages])
        .mount("/", routes![received_message])
        .mount("/", routes![temp])
}

fn main() {
    mounts().launch();
}

// use cargo test -- --nocapture to get output
#[cfg(test)]
mod tests{

    use rocket::http::RawStr;
    use rocket::http::{ContentType};

    use super::mounts;
    use super::Message;
    use rocket::local::Client;
    use rocket::http::Status;
    use reqwest;
    use rocket_contrib::json::Json;


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
        //let client = Client::new(rocket()).expect("valid rocket instance");

        let request_client = reqwest::Client::new();
        let res = request_client.post("http://localhost:8000/message/received")
        .json(&message)
        .send();

        //println!("response: {}", res);
    }

    #[test]
    fn temp(){
        let message = Message{
            message: String::from("this is my test string"),
            user: String::from("test user"),
            complete: true
        };

        let body = Json(message);
        let client = Client::new(mounts()).expect("valid rocket instance");
        let result = client.post("/message/temp")
        .header(ContentType::JSON)
        .body(body)        
        .dispatch();
        /*let request_client = reqwest::blocking::Client::new();
        let res = request_client.post("http://localhost:8000/message/temp")
        .send()?;*/
        
        //println!("response: {}", result);

    }
}