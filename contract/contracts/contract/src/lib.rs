#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Env, String,
};

#[contract]
pub struct ProductWarrantyRegistry;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Warranty(String), // product_id
}

#[contracttype]
#[derive(Clone)]
pub struct Warranty {
    pub product_id: String,
    pub serial_number: String,
    pub owner: Address,
    pub issuer: Address,
    pub purchase_date: u64,
    pub warranty_end: u64,
    pub metadata_uri: String,
    pub active: bool,
}

#[contracterror]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum WarrantyError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    WarrantyAlreadyExists = 4,
    WarrantyNotFound = 5,
    WarrantyInactive = 6,
}

#[contractimpl]
impl ProductWarrantyRegistry {
    // Initialize the contract once with an admin
    pub fn init(env: Env, admin: Address) -> Result<(), WarrantyError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(WarrantyError::AlreadyInitialized);
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);

        Ok(())
    }

    // Register a new product warranty
    pub fn register_warranty(
        env: Env,
        product_id: String,
        serial_number: String,
        owner: Address,
        issuer: Address,
        purchase_date: u64,
        warranty_end: u64,
        metadata_uri: String,
    ) -> Result<(), WarrantyError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();

        let key = DataKey::Warranty(product_id.clone());

        if env.storage().persistent().has(&key) {
            return Err(WarrantyError::WarrantyAlreadyExists);
        }

        let warranty = Warranty {
            product_id,
            serial_number,
            owner,
            issuer,
            purchase_date,
            warranty_end,
            metadata_uri,
            active: true,
        };

        env.storage().persistent().set(&key, &warranty);
        Ok(())
    }

    // View warranty details
    pub fn get_warranty(env: Env, product_id: String) -> Result<Warranty, WarrantyError> {
        let key = DataKey::Warranty(product_id);
        env.storage()
            .persistent()
            .get(&key)
            .ok_or(WarrantyError::WarrantyNotFound)
    }

    // Transfer warranty ownership to another address
    pub fn transfer_warranty(
        env: Env,
        product_id: String,
        new_owner: Address,
    ) -> Result<(), WarrantyError> {
        let key = DataKey::Warranty(product_id);
        let mut warranty: Warranty = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(WarrantyError::WarrantyNotFound)?;

        if !warranty.active {
            return Err(WarrantyError::WarrantyInactive);
        }

        warranty.owner.require_auth();
        warranty.owner = new_owner;

        env.storage().persistent().set(&key, &warranty);
        Ok(())
    }

    // Mark a warranty inactive / void
    pub fn deactivate_warranty(env: Env, product_id: String) -> Result<(), WarrantyError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();

        let key = DataKey::Warranty(product_id);
        let mut warranty: Warranty = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(WarrantyError::WarrantyNotFound)?;

        warranty.active = false;
        env.storage().persistent().set(&key, &warranty);

        Ok(())
    }

    // Check if warranty is currently valid
    pub fn is_warranty_valid(env: Env, product_id: String) -> Result<bool, WarrantyError> {
        let key = DataKey::Warranty(product_id);
        let warranty: Warranty = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(WarrantyError::WarrantyNotFound)?;

        let now = env.ledger().timestamp();

        Ok(warranty.active && now <= warranty.warranty_end)
    }

    // Read admin
    pub fn get_admin_public(env: Env) -> Result<Address, WarrantyError> {
        Self::get_admin(&env)
    }

    fn get_admin(env: &Env) -> Result<Address, WarrantyError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(WarrantyError::NotInitialized)
    }
}
