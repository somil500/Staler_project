#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

// Struct to represent a single stage of the product's journey
#[contracttype]
#[derive(Clone)]
pub struct Stage {
    pub location: String,
    pub description: String,
    pub timestamp: String,
}

// Struct to represent a product
#[contracttype]
#[derive(Clone)]
pub struct Product {
    pub product_id: u64,
    pub creator: Address,
    pub name: String,
    pub origin: String,
    pub current_holder: Address,
    pub history: Vec<Stage>,
}

// Storage keys for products
#[contracttype]
pub enum ProductKey {
    Product(u64),
    Count,
}

#[contract]
pub struct ProductProvenanceTracker;

#[contractimpl]
impl ProductProvenanceTracker {
    pub fn register_product(
        env: Env,
        creator: Address,
        name: String,
        origin: String,
        initial_stage_description: String,
        timestamp: String,
    ) -> u64 {
        let mut count = env.storage().instance().get(&ProductKey::Count).unwrap_or(0);
        count += 1;

        let mut history = Vec::new(&env);
        history.push_back(Stage {
            location: origin.clone(),
            description: initial_stage_description,
            timestamp,
        });

        let product = Product {
            product_id: count,
            creator: creator.clone(),
            name,
            origin,
            current_holder: creator,
            history,
        };

        env.storage().instance().set(&ProductKey::Product(count), &product);
        env.storage().instance().set(&ProductKey::Count, &count);

        count
    }

    pub fn add_stage(
        env: Env,
        product_id: u64,
        new_holder: Address,
        location: String,
        description: String,
        timestamp: String,
    ) {
        let mut product: Product = env
            .storage()
            .instance()
            .get(&ProductKey::Product(product_id))
            .expect("Product not found");

        product.current_holder = new_holder;
        product.history.push_back(Stage {
            location,
            description,
            timestamp,
        });

        env.storage().instance().set(&ProductKey::Product(product_id), &product);
    }

    pub fn get_product(env: Env, product_id: u64) -> Product {
        env.storage()
            .instance()
            .get(&ProductKey::Product(product_id))
            .expect("Product not found")
    }
}
