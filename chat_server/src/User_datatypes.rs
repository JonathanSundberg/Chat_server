#![feature(proc_macro_hygiene, decl_macro)]
use parking_lot::Mutex;

use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    collections::HashMap,
    collections::HashSet,
    fs,
    fs::File,
    hash::{
        // collections::hash_map::DefaultHasher Requires these to hash
        Hash,
        Hasher,
    },
    path::PathBuf,
};
use uuid::Uuid;

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

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct UserRegister {
    pub user_name: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct User {
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub id: u64,
    pub conversations: Conversations,
}

impl User {

    pub fn _create_User_storage_directory(&self) -> Result<File, &str> {
        // TODO: Check if File::create creates necessary folders too
        let path_string = format!("Users/{}", self.id);
        let user_storage_path = PathBuf::from(&path_string);
        fs::create_dir_all(user_storage_path).unwrap();

        let conversations_file = format!("{}/conversations.json", path_string);
        let new_file = File::create(conversations_file).unwrap();
        serde_json::to_writer_pretty(&new_file, &Conversations::default())
            .expect("Could not write to the user conversation file");
        Ok(new_file)
    }

    pub fn get_user_dir(&self) -> String {
        return format!("Users/{}", self.id.clone());
    }

    pub fn read_user_conversations_file(&mut self){

        let path = format!("Users/{}/Conversations.json", self.id.clone());
        let f = File::open(&path);
        dbg!(&path);
        let f = match f{
            Ok(f) => f,
            Err(_) =>{
                self._create_User_storage_directory().unwrap()
            }
        };
        let user_conversations: Result<Conversations, serde_json::Error> =
            serde_json::from_reader(f);
        match user_conversations {
            Ok(content) => self.conversations = content,
            Err(_) => self.conversations = Conversations::default()
        }
    }

    pub fn save_user_conversations_file(&self) -> Result<(), String> {

        let path = format!("Users/{}/Conversations.json", self.id.clone());
        let new_file = File::create(path).unwrap();
        serde_json::to_writer_pretty(new_file, &self.conversations)
            .expect("Could not write to the users conversation file");
        Ok(())
        
    }

    pub fn add_conversation_to_user(&mut self, conversation_id: String) -> Result<(), &'static str>{
        self.read_user_conversations_file();
        let was_new = self.conversations.conv_hashset.insert(conversation_id);
        if was_new{
            self.save_user_conversations_file();
            Ok(())
        }
        else{
            Err("Id Already existed")
        }

        

    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDatabase {
    pub users: Mutex<HashMap<String, User>>,
    pub emails: Mutex<HashSet<String>>,
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
    pub fn save_users_to_file(&self) {
        let new_file = File::create("Users.json").unwrap();
        serde_json::to_writer_pretty(new_file, &self)
            .expect("Could not write to the Users.json file");
    }

    pub fn read_users_from_file(&mut self) {
        let f = File::open("Users.json").unwrap();
        let json_database: Result<UserDatabase, serde_json::Error> = serde_json::from_reader(f);
        match json_database {
            Ok(content) => *self = content,
            Err(_) => *self = UserDatabase::default(),
        }
    }

    pub fn _check_if_username_exists(&self, compare_name: &String) -> bool {
        self.users.lock().contains_key(compare_name)
    }

    pub fn _check_if_email_exists(&self, user: &String) -> bool {
        self.emails.lock().contains(user)
    }

    pub fn _add_conversation(&self, chat_id: &String) {
        // the conversation file holding the chat messages
        let path_string = format!("Conversations/{}", chat_id.clone());
        let conversations_dir = PathBuf::from(&path_string);
        fs::create_dir_all(conversations_dir).unwrap();

        let conversation_file = format!("{}/conversation.json", path_string);
        let new_file = File::create(conversation_file).unwrap();
        serde_json::to_writer_pretty(new_file, &Chat::default())
            .expect("Could not create the conversation file");
    }

    pub fn remove_user_from_database(&self, user: User) {
        if !self._check_if_email_exists(&user.email) {
            println!("Email does not exists");
            return;
        }
        self.users.lock().remove(&user.user_name);
        self.emails.lock().remove(&user.email);
        self.save_users_to_file();
    }

    pub fn write_user_to_database(&self, mut user: User) {
        if self._check_if_username_exists(&user.user_name) {
            println!("Username exists");
            return;
        }
        if self._check_if_email_exists(&user.email) {
            println!("Email exists");
            return;
        }
        user.id = user.email.to_hash();

        self.users
            .lock()
            .insert(user.user_name.clone(), user.clone());
        self.emails.lock().insert(user.email.clone());

        self.save_users_to_file();
    }

    pub fn register_new_user(&self, mut user: UserRegister) {
        if self._check_if_username_exists(&user.user_name) {
            println!("Username exists");
            return;
        }
        if self._check_if_email_exists(&user.email) {
            println!("Email exists");
            return;
        }

        let user_id = user.email.clone().to_hash();
        let new_user = User{
            user_name: user.user_name.clone(),
            email: user.email.clone(),
            password: user.password,
            id: user_id,
            conversations: Conversations::default()
        };

        self.users
            .lock()
            .insert(new_user.user_name.clone(), new_user.clone());
        self.emails.lock().insert(user.email.clone());

        self.save_users_to_file();
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Chat {
    pub messages: HashMap<String, Message>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Chats {
    pub chat_dict: HashMap<String, Chat>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Conversations {
    pub conv_hashset: HashSet<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub message: String,
    pub user: String,
    pub complete: bool,
}
