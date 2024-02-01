use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr};
use uuid::Uuid;

const EXP_PER_LEVEL: u8 = 100;


#[derive(Default)]
struct AppState {
    users: HashMap<String, User>,
    pets: HashMap<String, Pet>,
    pet_yards: HashMap<String, PetYard>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            pets: HashMap::new(),
            pet_yards: HashMap::new(),
        }
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
    // Basic user info
    uuid: String,
    join_timestamp: u64,
    username: String,
    email: String,
    // Password is hashed and salted
    password: String,

    // UUID of users's pets
    pets: Vec<String>,
    // UUIDs of user's pet yards
    owned_pet_yards: Vec<String>,
    // UUIDs of pet yards the user has joined
    joined_pet_yards: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Pet {
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
    fn new(name: String, species: String) -> Self {
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

    fn set_pet_yard(&mut self, pet_yard_uuid: String) {
        self.pet_yard = Some(pet_yard_uuid);
    }

    fn remove_pet_yard(&mut self) {
        self.pet_yard = None;
    }

    fn get_pet_yard(&self) -> Option<String> {
        self.pet_yard.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_species(&self) -> String {
        self.species.clone()
    }

    fn set_species(&mut self, species: String) {
        self.species = species;
    }

    fn get_level(&self) -> u128 {
        self.level
    }

    fn get_experience(&self) -> u8 {
        self.experience
    }

    fn add_experience(&mut self, experience: u8) {
        self.experience += experience;
        if self.experience >= EXP_PER_LEVEL {
            self.level += 1;
            self.experience = 0;
        }
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PetYard {
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
    fn new(name: String, owner_uuid: String) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            name,
            owner: owner_uuid,
            members: vec![],
            pets: vec![],
        }
    }

    fn add_member(&mut self, member_uuid: String) {
        if !self.members.contains(&member_uuid) {
            self.members.push(member_uuid);
        }
    }

    fn remove_member(&mut self, member_uuid: String) {
        self.members.retain(|uuid| uuid != &member_uuid);
    }

    fn get_members(&self) -> Vec<String> {
        // Return members along with the owner
        let mut members = self.members.clone();
        members.push(self.owner.clone());
        members
    }

    fn add_pet(&mut self, pet_uuid: String) {
        if !self.pets.contains(&pet_uuid) {
            self.pets.push(pet_uuid);
        }
    }

    fn remove_pet(&mut self, pet_uuid: String) {
        self.pets.retain(|uuid| uuid != &pet_uuid);
    }

    fn get_pets(&self) -> Vec<String> {
        self.pets.clone()
    }

    fn get_owner(&self) -> String {
        self.owner.clone()
    }
}
