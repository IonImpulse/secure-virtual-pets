use std::collections::HashMap;
use uuid::Uuid;


use crate::encryption::hash;

const EXP_PER_LEVEL: u8 = 100;


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct AppState {
    pub users: HashMap<String, User>,
    pub pets: HashMap<String, Pet>,
    pub pet_yards: HashMap<String, PetYard>,
    pub tokens: HashMap<String, UserToken>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            pets: HashMap::new(),
            pet_yards: HashMap::new(),
            tokens: HashMap::new(),
        }
    }

    pub fn create_token(&mut self, user: &User) -> String {
        let token = UserToken::new(user.uuid.clone(), Uuid::new_v4().to_string());
        self.tokens.insert(token.token.clone(), token.clone());

        token.token
    }

    /*
    
    User functions

     */

    pub fn get_user_by_uuid(&self, uuid: &str) -> Option<&User> {
        self.users.get(uuid)
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.users.values().find(|&user| user.username == username)
    }

    pub fn update_user(&mut self, user: User) {
        self.users.insert(user.uuid.clone(), user);
    }

    pub fn delete_user(&mut self, user: User) {
        // First, delete the user's pets
        for pet_uuid in user.pets.iter() {
            self.delete_pet(pet_uuid);
        }

        // Next, delete the user's pet yards
        for pet_yard_uuid in user.owned_pet_yards.iter() {
            self.delete_pet_yard(pet_yard_uuid);
        }

        // Finally, delete the user
        self.users.remove(&user.uuid);
    }

    /*
    
    Pet functions
    
     */

    pub fn get_pet_by_uuid(&self, uuid: &str) -> Option<&Pet> {
        self.pets.get(uuid)
    }

    pub fn update_pet(&mut self, pet: Pet) {
        self.pets.insert(pet.uuid.clone(), pet);
    }

    pub fn delete_pet(&mut self, uuid: &str) {
        // Remove the pet from any pet yards it is in
        for pet_yard in self.pet_yards.values_mut() {
            pet_yard.remove_pet(uuid.to_string());
        }

        // Finally, delete the pet
        self.pets.remove(uuid);
    }


    /*
    
    Pet yard functions
    
     */

    pub fn get_pet_yard_by_uuid(&self, uuid: &str) -> Option<&PetYard> {
        self.pet_yards.get(uuid)
    }

    pub fn update_pet_yard(&mut self, pet_yard: PetYard) {
        self.pet_yards.insert(pet_yard.uuid.clone(), pet_yard);
    }

    pub fn delete_pet_yard(&mut self, uuid: &str) {
        // First, remove the pet yard from all users
        for user in self.users.values_mut() {
            user.remove_owned_pet_yard(uuid.to_string());
            user.remove_joined_pet_yard(uuid.to_string());
        }

        // Next, remove all pets from the pet yard
        if let Some(pet_yard) = self.pet_yards.get(uuid) {
            let pet_uuids: Vec<String> = pet_yard.pets.iter().cloned().collect();

            // For each pet in the yard, just remove the pet yard
            for pet_uuid in pet_uuids {
                let mut pet = self.get_pet_by_uuid(&pet_uuid).unwrap().to_owned();

                pet.remove_pet_yard();

                // Update the pet
                self.update_pet(pet.clone());
            }
        }

        // Finally, delete the pet yard
        self.pet_yards.remove(uuid);
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct User {
    // Basic user info
    uuid: String,
    join_timestamp: u64,
    username: String,
    email: String,
    // Password is hashed and salted
    h_s_password: String,
    salt: String,
    // UUID of users's pets
    pets: Vec<String>,
    // UUIDs of user's pet yards
    owned_pet_yards: Vec<String>,
    // UUIDs of pet yards the user has joined
    joined_pet_yards: Vec<String>,
    // Chat logs with other users
    chat_logs: HashMap<String, Vec<DirectMessage>>,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        let salt = Uuid::new_v4().to_string();
        let salted_password = format!("{}{}", password, salt);
        let hashed_password = hash(&salted_password);

        Self {
            uuid: Uuid::new_v4().to_string(),
            join_timestamp: chrono::Utc::now().timestamp_millis() as u64,
            username,
            email,
            h_s_password: hashed_password,
            salt,
            pets: vec![],
            owned_pet_yards: vec![],
            joined_pet_yards: vec![],
            chat_logs: HashMap::new(),
        }
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn set_password(&mut self, password: String) {
        let salt = Uuid::new_v4().to_string();
        let salted_password = format!("{}{}", password, salt);
        let hashed_password = hash(&salted_password);

        self.h_s_password = hashed_password;
        self.salt = salt;
    }

    pub fn compare_password(&self, password: &str) -> bool {
        // Compare the hashed and salted password with the input password
        let salted_password = format!("{}{}", password, self.salt);
        let hashed_password = hash(&salted_password);

        // Wait a random amount of time to prevent timing attacks
        std::thread::sleep(std::time::Duration::from_millis(rand::random::<u64>() % 10));

        hashed_password == self.h_s_password
    }

    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    pub fn for_user(&self) -> String {
        serde_json::json!({
            "uuid": self.uuid,
            "join_timestamp": self.join_timestamp,
            "username": self.username,
            "email": self.email,
            "pets": self.pets,
            "owned_pet_yards": self.owned_pet_yards,
            "joined_pet_yards": self.joined_pet_yards,
            "chat_logs": self.chat_logs,
        }).to_string()
    }

    pub fn for_user_with_token(&self, token: String) -> String {
        serde_json::json!({
            "uuid": self.uuid,
            "join_timestamp": self.join_timestamp,
            "username": self.username,
            "email": self.email,
            "pets": self.pets,
            "owned_pet_yards": self.owned_pet_yards,
            "joined_pet_yards": self.joined_pet_yards,
            "chat_logs": self.chat_logs,
            "token": token,
        }).to_string()
    }

    pub fn for_public(&self) -> String {
        serde_json::json!({
            "uuid": self.uuid,
            "username": self.username,
            "pets": self.pets,
            "owned_pet_yards": self.owned_pet_yards,
        }).to_string()
    }

    pub fn add_pet(&mut self, pet_uuid: String) {
        if !self.pets.contains(&pet_uuid) {
            self.pets.push(pet_uuid);
        }
    }

    pub fn remove_pet(&mut self, pet_uuid: String) {
        self.pets.retain(|uuid| uuid != &pet_uuid);
    }

    pub fn add_owned_pet_yard(&mut self, pet_yard_uuid: String) {
        if !self.owned_pet_yards.contains(&pet_yard_uuid) {
            self.owned_pet_yards.push(pet_yard_uuid);
        }
    }

    pub fn remove_owned_pet_yard(&mut self, pet_yard_uuid: String) {
        self.owned_pet_yards.retain(|uuid| uuid != &pet_yard_uuid);
    }

    pub fn add_joined_pet_yard(&mut self, pet_yard_uuid: String) {
        if !self.joined_pet_yards.contains(&pet_yard_uuid) {
            self.joined_pet_yards.push(pet_yard_uuid);
        }
    }

    pub fn remove_joined_pet_yard(&mut self, pet_yard_uuid: String) {
        self.joined_pet_yards.retain(|uuid| uuid != &pet_yard_uuid);
    }


}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct UserToken {
    uuid: String,
    token: String,
    creation_timestamp: u64,
    expiration_timestamp: u64,
}

impl UserToken {
    pub fn new(uuid: String, token: String) -> Self {
        Self {
            uuid,
            token,
            creation_timestamp: chrono::Utc::now().timestamp_millis() as u64,
            expiration_timestamp: chrono::Utc::now().timestamp_millis() as u64 + 1000 * 60 * 60 * 24, // 24 hours
        }
    }

    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.expiration_timestamp > chrono::Utc::now().timestamp_millis() as u64
    }

    pub fn refresh(&mut self) {
        self.expiration_timestamp = chrono::Utc::now().timestamp_millis() as u64 + 1000 * 60 * 60 * 24; // 24 hours
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct DirectMessage {
    sender: String,
    receiver: String,
    encrypted_msg: String,
    timestamp: u64,
}

impl DirectMessage {
    pub fn new(sender: String, receiver: String, encrypted_msg: String) -> Self {
        Self {
            sender,
            receiver,
            encrypted_msg,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    pub fn decrypt(&self, _key: String) -> String {
        // Decrypt the message using the key
        self.encrypted_msg.clone() // Placeholder
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct Pet {
    uuid: String,
    name: String,
    image: u64,
    species: String,
    level: u128,
    experience: u8,
    // UUID of the pet yard the pet is in, or None if the pet is not in a pet yard
    pet_yard: Option<String>,
}

impl Pet {
    pub fn new(name: String, species: String, image: u64, pet_yard: Option<String>) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            name,
            image,
            species,
            level: 1,
            experience: 0,
            pet_yard,
        }
    }

    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    pub fn set_pet_yard(&mut self, pet_yard_uuid: String) {
        self.pet_yard = Some(pet_yard_uuid);
    }

    pub fn remove_pet_yard(&mut self) {
        self.pet_yard = None;
    }

    pub fn get_pet_yard(&self) -> Option<String> {
        self.pet_yard.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_species(&self) -> String {
        self.species.clone()
    }

    pub fn set_species(&mut self, species: String) {
        self.species = species;
    }

    pub fn get_level(&self) -> u128 {
        self.level
    }

    pub fn get_experience(&self) -> u8 {
        self.experience
    }

    pub fn add_experience(&mut self, experience: u8) {
        self.experience += experience;
        if self.experience >= EXP_PER_LEVEL {
            self.level += 1;
            self.experience = 0;
        }
    }

    pub fn get_image(&self) -> u64 {
        self.image
    }

    pub fn set_image(&mut self, image: u64) {
        self.image = image;
    }

    pub fn for_public(&self) -> String {
        serde_json::json!({
            "uuid": self.uuid,
            "name": self.name,
            "image": self.image,
            "species": self.species,
            "level": self.level,
            "experience": self.experience,
            "in_pet_yard": self.pet_yard.is_some(),
        }).to_string()
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PetYard {
    uuid: String,
    name: String,
    image: u64,
    // UUID of the user who owns the pet yard
    owner: String,
    // UUIDs of the users who have joined the pet yard
    members: Vec<String>,
    // UUIDs of the pets in the pet yard
    pets: Vec<String>,
}

impl PetYard {
    pub fn new(name: String, owner_uuid: String, image: u64) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            name,
            image,
            owner: owner_uuid,
            members: vec![],
            pets: vec![],
        }
    }

    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_image(&mut self, image: u64) {
        self.image = image;
    }

    pub fn get_image(&self) -> u64 {
        self.image
    }

    pub fn add_member(&mut self, member_uuid: String) {
        if !self.members.contains(&member_uuid) {
            self.members.push(member_uuid);
        }
    }

    pub fn remove_member(&mut self, member_uuid: String) {
        self.members.retain(|uuid| uuid != &member_uuid);
    }

    pub fn get_members(&self) -> Vec<String> {
        // Return members along with the owner
        let mut members = self.members.clone();
        members.push(self.owner.clone());
        members
    }

    pub fn add_pet(&mut self, pet_uuid: String) {
        if !self.pets.contains(&pet_uuid) {
            self.pets.push(pet_uuid);
        }
    }

    pub fn remove_pet(&mut self, pet_uuid: String) {
        self.pets.retain(|uuid| uuid != &pet_uuid);
    }

    pub fn get_pets(&self) -> Vec<String> {
        self.pets.clone()
    }

    pub fn get_owner(&self) -> String {
        self.owner.clone()
    }

    pub fn for_public(&self) -> String {
        serde_json::json!({
            "uuid": self.uuid,
            "name": self.name,
            "image": self.image,
            "owner": self.owner,
            "num_members": self.members.len(),
            "num_pets": self.pets.len(),
        }).to_string()
    }
}
