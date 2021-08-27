#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
use serde::Deserialize;
use serde_json;
use rocket_contrib::json::Json;
#[macro_use] extern crate rocket;


#[derive(FromForm, Deserialize)]
struct Message{
    message: String,
    user: String,

}



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/message/update/<sent_string>")]
fn update_messages(sent_string: &RawStr) -> String{
    format!("This is the string:, {}", sent_string.as_str())
}

#[post("/message/received", format = "json")]
fn received_message() -> String{
    String::from("asd")
}


fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![update_messages])
        .mount("/", routes![received_message])
}

fn main() {
    rocket().launch();
}

// use cargo test -- --nocapture to get output
#[cfg(test)]
mod tests{
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn update_messages(){
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        //assert_eq!(response.body_string(), Some("Hello, world!".into()));

        println!("message string: {}", response.body_string().unwrap());
    }

    fn received_message(){

    }
}