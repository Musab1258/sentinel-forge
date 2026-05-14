use soroban_sdk::{Address, Env};

pub fn register(e: Env, admin: Address, user: Address) {
    admin.require_auth();
    registry::set_member(&e, &user);
}
