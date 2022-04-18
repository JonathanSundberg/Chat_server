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
pub struct UserRegister { // The minimum required information to register a user
    pub user_name: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct User { // The information stored for a user
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub id: u64,
    pub conversations: UserConversations,
    pub message_queue: HashSet<Message>,
}

impl User {
    pub fn _create_User_storage_directory(&self) -> Result<File, &str> {
        let path_string = format!("Users/{}", self.id);
        let user_storage_path = PathBuf::from(&path_string);
        fs::create_dir_all(user_storage_path).unwrap();

        let conversations_file = format!("{}/conversations.json", path_string);
        let new_file = File::create(conversations_file).unwrap();
        serde_json::to_writer_pretty(&new_file, &UserConversations::default())
            .expect("Could not write to the user conversation file");
        Ok(new_file)
    }

    pub fn get_user_dir(&self) -> String {
        return format!("Users/{}", self.id.clone());
    }

    pub fn read_user_conversations_file(&mut self) { // reading the users conversation from saved filed
        let path = format!("Users/{}/Conversations.json", self.id.clone());
        let f = File::open(&path);
        dbg!(&path);
        let f = match f {
            Ok(f) => f,
            Err(_) => self._create_User_storage_directory().unwrap(),
        };
        let user_conversations: Result<UserConversations, serde_json::Error> =
            serde_json::from_reader(f);
        match user_conversations {
            Ok(content) => self.conversations = content,
            Err(_) => self.conversations = UserConversations::default(),
        }
    }

    pub fn save_user_conversations_file(&self) -> Result<(), String> { // Writing the users conversations to file. Only by IDs
        let path = format!("Users/{}/Conversations.json", self.id.clone());
        let new_file = File::create(path).unwrap();
        serde_json::to_writer_pretty(new_file, &self.conversations)
            .expect("Could not write to the users conversation file");
        Ok(())
    }

    pub fn add_conversation_to_user(
        &mut self,
        conversation_id: String,
    ) -> Result<(), &'static str> {
        self.read_user_conversations_file();
        let was_new = self.conversations.conv_list.insert(conversation_id);
        if was_new {
            self.save_user_conversations_file();
            Ok(())
        } else {
            Err("Id Already existed")
        }
    }

    pub fn add_to_message_queue(mut self, message: Message) {
        self.message_queue.insert(message);
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDatabase { // stores all users, all emails and all usernames
    pub users: Mutex<HashMap<u64, User>>,
    pub emails: Mutex<HashSet<String>>,
    pub usernames: Mutex<HashSet<String>>,
}

impl Default for UserDatabase {
    fn default() -> Self {
        UserDatabase {
            users: Mutex::new(HashMap::default()),
            emails: Mutex::new(HashSet::default()),
            usernames: Mutex::new(HashSet::default()),
        }
    }
}

impl UserDatabase {
    pub fn save_users_to_file(&self) { // TODO: make private?
        let new_file = File::create("Users.json").unwrap();
        serde_json::to_writer_pretty(new_file, &self)
            .expect("Could not write to the Users.json file");
    }

    pub fn read_users_from_file(&mut self) { // TODO: make private?
        let f = File::open("Users.json").unwrap();
        let json_database: Result<UserDatabase, serde_json::Error> = serde_json::from_reader(f);
        match json_database {
            Ok(content) => *self = content,
            Err(e) => {
                println!("Could not read users from file: {}", e);
                *self = UserDatabase::default()
            }
        }
    }

    pub fn _check_if_username_exists(&self, compare_name: &String) -> bool { // TODO: make private?
        self.usernames.lock().contains(compare_name)
    }

    pub fn _check_if_email_exists(&self, user: &String) -> bool { // TODO: make private?
        self.emails.lock().contains(user)
    }

    pub fn remove_user_from_database(&self, user: User) -> Result<(), ()>{
        if !self._check_if_email_exists(&user.email) {
            println!("Email does not exists");
            return Err(());
        }
        self.users.lock().remove(&user.id);
        self.emails.lock().remove(&user.email);
        self.save_users_to_file();
        Ok(())
    }

    pub fn write_user_to_database(&self, mut user: User) { // TODO: make private?
        if self._check_if_username_exists(&user.user_name) {
            println!("Username exists");
            return;
        }
        if self._check_if_email_exists(&user.email) {
            println!("Email exists");
            return;
        }
        user.id = user.email.to_hash();

        self.users.lock().insert(user.id.clone(), user.clone());
        self.emails.lock().insert(user.email.clone());

        self.save_users_to_file();
    }

    pub fn register_new_user(&self, user: UserRegister) {
        if self._check_if_username_exists(&user.user_name) {
            println!("Username exists");
            return;
        }
        if self._check_if_email_exists(&user.email) {
            println!("Email exists");
            return;
        }

        let user_id = user.email.clone().to_hash();
        let new_user = User {
            user_name: user.user_name.clone(),
            email: user.email.clone(),
            password: user.password,
            id: user_id,
            conversations: UserConversations::default(),
            message_queue: HashSet::default(),
        };

        self.users
            .lock()
            .insert(new_user.id.clone(), new_user.clone());
        self.emails.lock().insert(user.email.clone());

        self.save_users_to_file();
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Conversations { // hashmap which holds a string with a unique id for the conversation, and a hashset with all users in said conversation by their IDs
    pub all_conversations: HashMap<String, UsersInConversationByID>,
}


#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct UsersInConversationByID { // Hashset which holds the user id of all users in a conversation
    pub conversation_users: HashSet<u64>,
}

impl UsersInConversationByID {
    pub fn deliver_message_to_concerners(&self, message: Message, userdatabase: &UserDatabase) { // Delivers the sent message to all stored users
        let mut all_users = userdatabase.users.lock();
        for conversation_user in self.conversation_users.iter() {
            all_users
                .get_mut(conversation_user)
                .unwrap()
                .message_queue
                .insert(message.clone());
        }
        dbg!(all_users);
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct CreateConversation { // instructions sent from client to create a conversation
    pub user_id: String,
    pub users_to_invite: Vec<String>,
    pub public: bool,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct UserConversations { // hashset that keeps track of all the conversation the user is part of
    pub conv_list: HashSet<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Message {
    pub message: String,
    pub user: String,
    pub conversation_id: String,
}
