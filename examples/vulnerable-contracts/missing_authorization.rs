use soroban_sdk::{Address, Env};

pub fn transfer_admin(e: Env, new_admin: Address) {
    storage::set_admin(&e, &new_admin);
}
