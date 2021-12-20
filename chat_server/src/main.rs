#![feature(proc_macro_hygiene, decl_macro)]
use parking_lot::Mutex;
use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::fs;
use std::hash::{Hash, Hasher}; // collections::hash_map::DefaultHasher Requires these to hash
use std::io::BufReader;
use std::str::FromStr;
use std::{
    collections::hash_map::DefaultHasher, collections::HashMap, collections::HashSet, fs::File,
    path::PathBuf,
};
use uuid::Uuid;

#[macro_use]
extern crate rocket;

trait Hashable {
    fn to_hash(&self) -> u64;
}

impl Hashable for String {
    fn to_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Chat {
    messages: HashMap<String, Message>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Chats {
    chat_dict: HashMap<String, Chat>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Conversations {
    conversations: HashSet<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    message: String,
    user: String,
    complete: bool,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
struct User {
    user_name: String,
    password: String,
    email: String,
    id: Option<u64>,
}

impl User {
    fn get_user_dir(&self) -> Result<String, String> {
        if self.id.is_some() {
            return Ok(format!("Users/{}", self.id.clone().unwrap()));
        } else {
            return Err("User has no id".to_string());
        }
    }

    fn read_user_conversations_file(&self) -> Result<Conversations, String> {
        if self.id.is_some() {
            let path = format!("Users/{}/Conversations.json", self.id.clone().unwrap());
            let f = File::open(path).unwrap();
            let user_conversations: Result<Conversations, serde_json::Error> =
                serde_json::from_reader(f);
            match user_conversations {
                Ok(content) => Ok(content),
                Err(_) => Err("Could not open users conversations file".to_string()),
            }
        } else {
            return Err("User has no id".to_string());
        }
    }

    fn save_user_conversations_file(&self, conversations: Conversations) -> Result<(), String> {
        if self.id.is_some() {
            let path = format!("Users/{}/Conversations.json", self.id.clone().unwrap());
            let new_file = File::create(path).unwrap();
            serde_json::to_writer_pretty(new_file, &conversations)
                .expect("Could not write to the users conversation file");
            Ok(())
        } else {
            return Err("User has no id".to_string());
        }
    }

    fn add_conversation_to_user(&self, conversation_id: u64) {}
}

#[derive(Deserialize, Serialize, Debug)]
struct UserDatabase {
    users: Mutex<HashMap<String, User>>,
    emails: Mutex<HashSet<String>>,
}

impl Default for UserDatabase {
    fn default() -> Self {
        UserDatabase {
            users: Mutex::new(HashMap::default()),
            emails: Mutex::new(HashSet::default()),
        }
    }
}

impl UserDatabase {
    fn save_users_to_file(&self) {
        let new_file = File::create("Users.json").unwrap();
        serde_json::to_writer_pretty(new_file, &self)
            .expect("Could not write to the Users.json file");
    }

    fn read_users_from_file(&mut self) {
        let f = File::open("Users.json").unwrap();
        let json_database: Result<UserDatabase, serde_json::Error> = serde_json::from_reader(f);
        match json_database {
            Ok(content) => *self = content,
            Err(_) => *self = UserDatabase::default(),
        }
    }

    fn _check_if_username_exists(&self, compare_name: &User) -> bool {
        self.users.lock().contains_key(&compare_name.user_name)
    }

    fn _check_if_email_exists(&self, user: &User) -> bool {
        self.emails.lock().contains(&user.email)
    }

    fn _add_conversation(&self, chat_id: &String) {
        // the conversation file holding the chat messages
        let path_string = format!("Conversations/{}", chat_id.clone());
        let conversations_dir = PathBuf::from(&path_string);
        fs::create_dir_all(conversations_dir).unwrap();

        let conversation_file = format!("{}/conversation.json", path_string);
        let new_file = File::create(conversation_file).unwrap();
        serde_json::to_writer_pretty(new_file, &Chat::default())
            .expect("Could not create the conversation file");
    }

    fn _create_User_storage_directory(&self, user: User) -> Result<(), &str> {
        let id = if user.id.is_some() {
            user.id.unwrap()
        } else {
            println!("User does not have an ID, returning");
            return Err("User does not have an ID, returning");
        };
        let path_string = format!("Users/{}", id);
        let user_storage_path = PathBuf::from(&path_string);
        fs::create_dir_all(user_storage_path).unwrap();
        let conversations_file = format!("{}/conversations.json", path_string);
        let new_file = File::create(conversations_file).unwrap();
        serde_json::to_writer_pretty(new_file, &Conversations::default())
            .expect("Could not write to the user conversation file");
        Ok(())
    }

    fn remove_user_from_database(&self, user: User) {
        if !self._check_if_email_exists(&user) {
            println!("Email does not exists");
            return;
        }
        self.users.lock().remove(&user.user_name);
        self.emails.lock().remove(&user.email);
        self.save_users_to_file();
    }

    fn write_user_to_database(&self, mut user: User) {
        if self._check_if_username_exists(&user) {
            println!("Username exists");
            return;
        }
        if self._check_if_email_exists(&user) {
            println!("Email exists");
            return;
        }
        user.id = Some(user.email.to_hash());

        self.users
            .lock()
            .insert(user.user_name.clone(), user.clone());
        self.emails.lock().insert(user.email.clone());

        self.save_users_to_file();
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/message/update/<sent_string>")]
fn update_messages(sent_string: &RawStr) -> String {
    format!("This is the string:, {}", sent_string.as_str())
}

// curl -X POST -H "Content-Type: application/json" -d @post_json.json http://localhost:8000/message/received  too test
#[post("/message/received", format = "json", data = "<message>")]
fn received_message(message: Json<Message>) -> String {
    println!("message: {}", &message.message);
    format!("We are getting a post request!")
}

#[post("/message/register", format = "json", data = "<user>")]
fn register_user(user: Json<User>, user_database: State<UserDatabase>) -> String {
    println!("We are registering a user!");
    user_database.write_user_to_database(user.into_inner());
    format!("User registered!")
}

#[post("/message/remove", format = "json", data = "<user>")]
fn remove_user(user: Json<User>) -> String {
    println!("We are removing a user!");
    //remove_user_from_database(user.into_inner());
    format!("User removed!")
}

fn _get_file_if_exists_else_create_empty(filepath: PathBuf) -> File {
    if filepath.exists() {
        return File::open(filepath).unwrap();
    } else {
        return File::create(filepath).unwrap();
    };
}

fn _parse_database_file(json_file: File) -> Option<UserDatabase> {
    let reader = BufReader::new(json_file);
    let json_database: Result<UserDatabase, serde_json::Error> = serde_json::from_reader(reader);
    match json_database {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

fn _get_users_database() -> UserDatabase {
    let filepath = PathBuf::from_str("Users.json").unwrap();
    let database_file = _get_file_if_exists_else_create_empty(filepath);
    let json_database = _parse_database_file(database_file);

    let user_database = match json_database {
        Some(content) => content,
        None => UserDatabase::default(),
    };
    user_database
}

fn mounts(user_database: UserDatabase) -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![update_messages])
        .mount("/", routes![received_message])
        .mount("/", routes![register_user])
        .mount("/", routes![remove_user])
        .manage(user_database)
}

fn initialze() -> UserDatabase {
    let mut user_database = UserDatabase::default();
    user_database.read_users_from_file();
    user_database
}

fn main() {
    let user_database = initialze();
    mounts(user_database).launch();
}

// use cargo test -- --nocapture to get output
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _write_userdatabase_to_file_test() {
        let id1 = Some(1111);
        let id2 = Some(11112);
        let temp_user_1 = User {
            user_name: "my_user".to_string(),
            password: "testing_my_password".to_string(),
            email: "test_email@trying.com".to_string(),
            id: id1,
        };
        let temp_user_2 = User {
            user_name: "second_user".to_string(),
            password: "Another_password".to_string(),
            email: "another_email@trying.com".to_string(),
            id: id2,
        };

        let mut user_database = UserDatabase::default();
        user_database.read_users_from_file();

        {
            // scope to make the rwlock drop before saving to file
            let mut map = user_database.users.lock();
            let mut set = user_database.emails.lock();

            map.insert(temp_user_1.user_name.clone(), temp_user_1.clone());
            set.insert(temp_user_1.email.clone());
        }

        user_database.save_users_to_file();
        println!("{:?}", user_database);
    }

    #[test]
    fn remove_user_from_database_test() {
        let id1 = Some(3232323);
        let temp_user_1 = User {
            user_name: "my_user".to_string(),
            password: "testing_my_password".to_string(),
            email: "test_email@trying.com".to_string(),
            id: id1,
        };
        let mut user_database = UserDatabase::default();
        user_database.read_users_from_file();
        user_database.remove_user_from_database(temp_user_1);
    }

    #[test]
    fn _create_User_storage_directory_test() {
        let mut temp_user_1 = User {
            user_name: "my_user".to_string(),
            password: "testing_my_password".to_string(),
            email: "test_emaild@trying.com".to_string(),
            id: None,
        };
        temp_user_1.id = Some(temp_user_1.email.to_hash());
        dbg!(&temp_user_1);
        let user_database = UserDatabase::default();
        let _result = user_database._create_User_storage_directory(temp_user_1);
    }

    #[test]
    fn create_conversation_test() {
        let mut temp_user_1 = User {
            user_name: "my_user".to_string(),
            password: "testing_my_password".to_string(),
            email: "test_emaild@trying.com".to_string(),
            id: None,
        };
        let conversation_id = Uuid::new_v4().to_string();
        temp_user_1.id = Some(temp_user_1.email.to_hash());
        dbg!(&temp_user_1);

        let user_database = UserDatabase::default();
        let _result = user_database._add_conversation(&conversation_id);
    }

    #[test]
    fn hashing_test() {
        let a = "a".to_string();
        dbg!(a.to_hash());
    }

    #[test]
    fn update_messages() {
        /*let client = Client::new(mounts()).expect("valid rocket instance");
        let mut response = client
            .get("/message/update/Testing_a_string_here")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        //assert_eq!(response.body_string(), Some("Hello, world!".into()));

        println!("message string: {}", response.body_string().unwrap());*/
    }

    #[test]
    fn received_message() {
        /*let message = Message {
            message: String::from("this is my test string"),
            user: String::from("test user"),
            complete: true,
        };
        let body = serde_json::to_string(&message).unwrap();

        let client = Client::new(mounts()).expect("valid rocket instance");
        let _result = client
            .post("/message/received")
            .header(ContentType::JSON)
            .body(&body)
            .dispatch();

        //println!("response: {}", res);*/
    }
}
