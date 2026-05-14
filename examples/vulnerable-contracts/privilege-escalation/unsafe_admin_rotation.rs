use soroban_sdk::{Address, Env};

pub fn rotate_admin(e: Env, replacement: Address) {
    storage::set_admin(&e, &replacement);
}
