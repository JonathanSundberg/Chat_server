#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::json::Json;
use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{
        // collections::hash_map::DefaultHasher Requires these to hash
        Hash,
        Hasher,
    },
    io::BufReader,
    path::PathBuf,
    str::FromStr,
};
use uuid::Uuid;

mod User_datatypes;
use User_datatypes::*;

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

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// curl -X POST -H "Content-Type: application/json" -d @post_json.json http://localhost:8000/message/received  too test
#[post("/message/received", format = "json", data = "<message>")]
fn received_message(user_database: State<UserDatabase>, conversations: State<Conversations>, message: Json<Message>) -> String {
    println!("message: {}", &message.message);

    let convo = conversations.all_conversations.get_mut(&message.conversation_id.clone()).unwrap();
    convo.deliver_message_to_concerners(message.into_inner(), user_database);
    
    format!("We are recieving a message")
}

#[post("/message/register", format = "json", data = "<user>")]
fn register_user(user: Json<UserRegister>, user_database: State<UserDatabase>) -> String {
    println!("We are registering a user!");
    user_database.register_new_user(user.into_inner());
    format!("User registered!")
}

#[post("/message/remove", format = "json", data = "<user>")]
fn remove_user(user: Json<User>) -> String {
    println!("We are removing a user!");
    //remove_user_from_database(user.into_inner());
    format!("User removed!")
}

#[post(
    "/conversation/create_conversation",
    format = "json",
    data = "<creation_struct>"
)]
fn create_conversation(
    user_database: State<UserDatabase>,
    creation_struct: Json<CreateConversation>,
) -> String {
    format!("We are creating a conversation")
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

fn mounts(user_database: UserDatabase, conversations: Conversations) -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![received_message])
        .mount("/", routes![register_user])
        .mount("/", routes![remove_user])
        .manage(user_database)
        .manage(conversations)
}

fn initialze_user_database() -> UserDatabase {
    let mut user_database = UserDatabase::default();
    user_database.read_users_from_file();
    user_database
}

fn main() {
    let user_database = initialze_user_database();
    let conversations = Conversations::default();
    mounts(user_database, conversations).launch();
}

// use cargo test -- --nocapture to get output
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    // UserDatabase tests
    #[test]
    fn _write_userdatabase_to_file_test() {
        let id1 = 9547029640726372498;
        let id2 = 9547029640726372490;
        let temp_user_1 = User {
            user_name: "my_user".to_string(),
            password: "testing_my_password".to_string(),
            email: "test_email@trying.com".to_string(),
            id: id1,
            conversations: UserConversations::default(),
            messageQueue: HashSet::default(),
        };
        let temp_user_2 = User {
            user_name: "second_user".to_string(),
            password: "Another_password".to_string(),
            email: "another_email@trying.com".to_string(),
            id: id2,
            conversations: UserConversations::default(),
            messageQueue: HashSet::default(),
        };

        let mut user_database = UserDatabase::default();
        user_database.read_users_from_file();

        {
            // scope to make the rwlock drop before saving to file
            let mut map = user_database.users.lock();
            let mut set = user_database.emails.lock();

            map.insert(id1, temp_user_1.clone());
            set.insert(temp_user_1.email.clone());
        }

        user_database.save_users_to_file();
        println!("{:?}", user_database);
    }

    #[test]
    fn remove_user_from_database_test() {
        let id1 = 9547029640726372498;
        let temp_user_1 = User {
            user_name: "my_user".to_string(),
            password: "testing_my_password".to_string(),
            email: "test_email@trying.com".to_string(),
            id: id1,
            conversations: UserConversations::default(),
            messageQueue: HashSet::default(),
        };
        let mut user_database = UserDatabase::default();
        user_database.read_users_from_file();
        user_database.remove_user_from_database(temp_user_1);
    }

    // User tests
    #[test]
    fn _create_User_storage_directory_test() {
        let id1 = 9547029640726372498;
        let mut temp_user_1 = User {
            user_name: "my_user".to_string(),
            password: "testing_my_password".to_string(),
            email: "test_email@trying.com".to_string(),
            id: id1,
            conversations: UserConversations::default(),
            messageQueue: HashSet::default(),
        };
        temp_user_1.id = temp_user_1.email.to_hash();
        dbg!(&temp_user_1);
        temp_user_1._create_User_storage_directory();
    }

    #[test]
    fn add_conversation_to_user_test() {
        let id1 = 9547029640726372498;
        let mut temp_user_1 = User {
            user_name: "my_user".to_string(),
            password: "testing_my_password".to_string(),
            email: "test_email@trying.com".to_string(),
            id: id1,
            conversations: UserConversations::default(),
            messageQueue: HashSet::default(),
        };
        temp_user_1.id = temp_user_1.email.to_hash();
        let conversation_id = Uuid::new_v4().to_string();
        let result = temp_user_1.add_conversation_to_user(conversation_id);
        dbg!(temp_user_1);
    }

    // Rocket tests

    #[test]
    fn received_message() {}
}
