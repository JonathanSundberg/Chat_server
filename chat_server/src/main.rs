#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/message/<sent_string>")]
fn get_messages(sent_string: &RawStr) -> String{
    format!("This is the string:, {}", sent_string.as_str())
}

#[get("/message/<sent_string>")]
fn send_message(sent_string: &RawStr) -> String{
    format!("This is the string:, {}", sent_string.as_str())
}


fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![get_messages])
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
    fn get_messages(){
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        //assert_eq!(response.body_string(), Some("Hello, world!".into()));

        println!("message string: {}", response.body_string().unwrap());
    }

    fn send_message(){

    }
}