use crate::config::*;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use sled::Db;

// Abstraction to perform hash an interface with the db
pub fn user_validate(user: &str, pass: &str, db: &Db) -> bool {
    let db = db.open_tree(USER_TREE_NAME).unwrap();
    return match db.get(user) {
        Ok(Some(db_hash_bytes)) => {
            let db_hash = String::from_utf8(db_hash_bytes.to_vec()).unwrap();

            let parsed_hash =
                PasswordHash::new(&db_hash).unwrap_or_else(|_| panic!("Invalid hash"));

            match Argon2::default().verify_password(pass.as_bytes(), &parsed_hash) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        _ => false,
    };
}

pub fn user_insert(user: &str, pass: &str, db: &Db) -> bool {
    let db = db.open_tree(USER_TREE_NAME).unwrap();
    // Assert db does not contain user
    match db.contains_key(user) {
        Ok(true) => return false,
        _ => {}
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(pass.as_bytes(), &salt).unwrap();

    // Insert user and hash into db
    let hash_bytes = hash.serialize();
    let phc_bytes = hash_bytes.as_bytes();

    db.insert(user, phc_bytes).unwrap();
    true
}

/// Remove a user from the db
pub fn user_remove(user: &str, db: &Db) -> bool {
    let mut flag = false;
    for subtree_name in [USER_TREE_NAME, USER_ID_NAME] {
        let subtree = db.open_tree(subtree_name).unwrap();
        match subtree.contains_key(user).unwrap() {
            true => {
                subtree.remove(user).unwrap();
                flag = true;
            }
            _ => {}
        };
    }
    flag
}

/// Get the uuid of a user, if it does not exist, create it
pub fn get_user_uuid(user: &str, db: &Db) -> String {
    let db = db.open_tree(USER_ID_NAME).unwrap();
    match db.get(&user).unwrap() {
        Some(uuid) => String::from_utf8(uuid.to_vec()).unwrap(),
        None => {
            let uuid = uuid::Uuid::new_v4().to_string();
            db.insert(&user, uuid.as_bytes()).unwrap();
            uuid
        }
    }
}

pub fn dump_users(db: &Db) {
    for subtree_name in [USER_TREE_NAME, USER_ID_NAME] {
        let subtree = db.open_tree(subtree_name).unwrap();
        for row in subtree.iter() {
            let (key, value) = row.unwrap();
            println!(
                "{}: {}",
                String::from_utf8(key.to_vec()).unwrap(),
                String::from_utf8(value.to_vec()).unwrap()
            );
        }
    }
}
