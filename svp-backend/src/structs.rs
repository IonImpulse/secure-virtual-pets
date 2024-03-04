use axum::http::header::Keys;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr};
use uuid::Uuid;
use chrono;

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

    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        for user in self.users.values() {
            if user.username == username {
                return Some(user);
            }
        }
        None
    }

    pub fn create_token(&mut self, user: &User) -> String {
        let token = UserToken::new(user.uuid.clone(), Uuid::new_v4().to_string());
        self.tokens.insert(token.token.clone(), token.clone());

        token.token
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

    pub fn compare_password(&self, password: &str) -> bool {
        // Compare the hashed and salted password with the input password
        let salted_password = format!("{}{}", password, self.salt);
        let hashed_password = hash(&salted_password);

        hashed_password == self.h_s_password
    }

    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
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

    pub fn decrypt(&self, key: String) -> String {
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
    pub fn new(name: String, species: String) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            name,
            image: 0,
            species,
            level: 1,
            experience: 0,
            pet_yard: None,
        }
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
}


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct PetYard {
    uuid: String,
    name: String,
    // UUID of the user who owns the pet yard
    owner: String,
    // UUIDs of the users who have joined the pet yard
    members: Vec<String>,
    // UUIDs of the pets in the pet yard
    pets: Vec<String>,
}

impl PetYard {
    pub fn new(name: String, owner_uuid: String) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            name,
            owner: owner_uuid,
            members: vec![],
            pets: vec![],
        }
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
}
