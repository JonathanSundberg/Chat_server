#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
use serde::{Deserialize, Serialize, de::Error};
use serde_json;
use std::{collections::HashMap, collections::HashSet, fs::File, path::PathBuf, result};
use rocket_contrib::json::{self, Json};
use std::io::BufReader;
use std::str::FromStr;
use std::fs;
use std::sync::RwLock;

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

#[derive(Deserialize, Serialize, Debug)]
struct UserDatabase{
    users: RwLock<HashMap<String, User>>,
    emails: RwLock<HashSet<String>>
}

impl Default for UserDatabase{
    fn default() -> Self{
        UserDatabase {
            users: RwLock::new(HashMap::default()),
            emails: RwLock::new(HashSet::default())
        }
    }
}

impl UserDatabase{

    fn get_variables_as_tuple(&self) -> (HashMap<String,User>, HashSet<String>){
        (self.users.read().unwrap().clone(),
         self.emails.read().unwrap().clone())
    }

    // make mutexed with rwlock
    fn save_to_file(&self, path: &PathBuf){

        println!("Wahoo!");
        let self_variables = self.get_variables_as_tuple();
        println!("Yepeee!");
        let new_file = File::create("Users.json").unwrap();
        println!("wo");
        serde_json::to_writer_pretty(new_file, &self_variables).expect("Could not write to the Users.json file");

    }

    fn read_from_file(&self, path: &PathBuf){

        /*let f = File::open(path).unwrap();

        let json_database: Result<UserDatabase, serde_json::Error> = serde_json::from_reader(f);
        match json_database{
            Ok(content) => Some(content),
            Err(_) => None
        }*/
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/message/update/<sent_string>")]
fn update_messages(sent_string: &RawStr) -> String{
    format!("This is the string:, {}", sent_string.as_str())
}

// curl -X POST -H "Content-Type: application/json" -d @post_json.json http://localhost:8000/message/received  too test
#[post("/message/received", format = "json", data = "<message>")]
fn received_message(message: Json<Message>) -> String {
    println!("message: {}", &message.message);
    format!("We are getting a post request!")
}

#[post("/message/register", format = "json", data = "<user>")]
fn register_user(user: Json<User>) -> String {
    println!("We are registering a user!");
    write_user_to_database(user.into_inner());
    format!("User registered!")
}
#[post("/message/remove", format = "json", data = "<user>")]
fn remove_user(user: Json<User>) -> String {
    println!("We are removing a user!");
    remove_user_from_database(user.into_inner());
    format!("User removed!")
}

fn _create_User_storage_directory(user_id: i32){
    let path_string = format!("Users/{}", user_id);
    let user_storage_path = PathBuf::from(path_string);
    fs::create_dir_all(user_storage_path).unwrap();
}

fn _check_if_username_exists(database: &UserDatabase, user: &User) -> bool{
    //database.users.contains_key(&user.user_name)
    true
}

fn _check_if_email_exists(database: &UserDatabase, user: &User) -> bool{
    //database.emails.contains(&user.email)
    true
}

fn _get_file_if_exists_else_create_empty(filepath: PathBuf) -> File{
    if filepath.exists(){
        return File::open(filepath).unwrap()
    }
    else{
        return File::create(filepath).unwrap()
    };
}


fn _parse_database_file(json_file: File) -> Option<UserDatabase>{
    let reader = BufReader::new(json_file);
    let json_database: Result<UserDatabase, serde_json::Error> = serde_json::from_reader(reader);
    match json_database{
        Ok(content) => Some(content),
        Err(_) => None
    }
}

fn _get_users_database() -> UserDatabase{
    let filepath = PathBuf::from_str("Users.json").unwrap();
    let database_file = _get_file_if_exists_else_create_empty(filepath);
    let json_database = _parse_database_file(database_file);

    let user_database = match json_database{
        Some(content) => content,
        None => {
            UserDatabase::default()
        }
    };
    user_database
}

fn write_user_to_database(user: User){
    
    let mut user_database = _get_users_database();

    if _check_if_username_exists(&user_database, &user){
        println!("Username exists");
        return;
    }
    if _check_if_email_exists(&user_database, &user){
        println!("Email exists");
        return;
    }

    //user_database.users.insert(user.user_name.clone(), user.clone());
    //user_database.emails.insert(user.email.clone());

    let new_file = File::create("Users.json").unwrap();
    serde_json::to_writer_pretty(new_file, &user_database).expect("Could not write to the Users.json file");

}

fn remove_user_from_database(user: User){
    let mut user_database = _get_users_database();

    if !_check_if_email_exists(&user_database, &user){
        println!("Email does not exists");
        return;
    }

    //user_database.users.remove(&user.user_name);
    //user_database.emails.remove(&user.email);

    let new_file = File::create("Users.json").unwrap();
    serde_json::to_writer_pretty(new_file, &user_database).expect("Could not write to the Users.json file");

}

fn create_conversation_list_file(){
    let conversation_list_path = PathBuf::from("Conversations.json");
    let conversation_file =_get_file_if_exists_else_create_empty(conversation_list_path);

}

fn mounts() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![update_messages])
        .mount("/", routes![received_message])
        .mount("/", routes![register_user])
        .mount("/", routes![remove_user])
}

fn initialze(){
    
}

fn main() {
    initialze();
    mounts().launch();
}


// use cargo test -- --nocapture to get output
#[cfg(test)]
mod tests{
    use rocket::http::{ContentType};

    use super::*;
    use rocket::local::Client;
    use rocket::http::Status;
    use std::fs::File;

    

    #[test]
    fn _write_userdatabase_to_file_test(){

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
        
        let user_database = UserDatabase::default();
        { // scope to make the rwlock drop before saving to file
            let mut map = user_database.users.write().unwrap();
            let mut set = user_database.emails.write().unwrap();
    
            map.insert(temp_user_2.user_name.clone(), temp_user_2.clone());
            set.insert(temp_user_2.email.clone());
        }

        user_database.save_to_file(&PathBuf::from("Users.json"));
        println!("{:?}", user_database);

        

    }


    #[test]
    fn _create_User_storage_directory_test(){
        let user_id = 12335;
        _create_User_storage_directory(user_id);
    }


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
    fn write_user_to_database_test() {

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
        let json_database = _parse_database_file(database_file);

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

        //json_content.users.insert(temp_user_2.user_name.clone(), temp_user_2.clone());
        //json_content.emails.insert(temp_user_2.email.clone());
        println!("{:?}", &json_content);

        let new_file = File::create("Users.json").unwrap();

        serde_json::to_writer_pretty(new_file, &json_content).expect("Could not write to the Users.json file");
    }

    #[test]
    fn remove_user_from_database_test(){
        let temp_user_1 = User{
            user_name: "my_user".to_string(),
            password: "testing_my_password".to_string(),
            email: "test_email@trying.com".to_string()
        };

        remove_user_from_database(temp_user_1);
    }

}