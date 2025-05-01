#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

// Structure to represent a product listing
#[contracttype]
#[derive(Clone)]
pub struct Product {
    pub product_id: u64,
    pub seller: Address,
    pub title: String,
    pub description: String,
    pub price: i128,
    pub sold: bool,
}

// Storage keys
#[contracttype]
pub enum ProductKey {
    Product(u64),
    Count,
}

#[contract]
pub struct P2PMarketplace;

#[contractimpl]
impl P2PMarketplace {
    // Create a new product listing
    pub fn create_listing(env: Env, seller: Address, title: String, description: String, price: i128) -> u64 {
        let mut count = env.storage().instance().get(&ProductKey::Count).unwrap_or(0);
        count += 1;

        let product = Product {
            product_id: count,
            seller,
            title,
            description,
            price,
            sold: false,
        };

        env.storage().instance().set(&ProductKey::Product(count), &product);
        env.storage().instance().set(&ProductKey::Count, &count);

        count
    }

    // View details of a product listing
    pub fn view_product(env: Env, product_id: u64) -> Product {
        env.storage().instance().get(&ProductKey::Product(product_id)).expect("Product not found")
    }

    // Buy a product and mark it as sold
    pub fn buy_product(env: Env, product_id: u64, _buyer: Address) {
        let mut product: Product = env.storage().instance().get(&ProductKey::Product(product_id)).expect("Product not found");

        if product.sold {
            panic!("Product already sold");
        }

        product.sold = true;
        env.storage().instance().set(&ProductKey::Product(product_id), &product);

        // Transfer payment logic can be added here
    }

    // View all products (listings)
    pub fn view_all_products(env: Env) -> Vec<Product> {
        let mut products = Vec::new(&env);
        let mut count = env.storage().instance().get(&ProductKey::Count).unwrap_or(0);

        while count > 0 {
            if let Some(product) = env.storage().instance().get(&ProductKey::Product(count)) {
                products.insert((count as u32) - 1, product); // fixed casting to u32
            }
            count -= 1;
        }

        products
    }
}
