use soroban_sdk::{Address, Env};

pub fn rotate_admin(e: Env, current_admin: Address, new_admin: Address) {
    current_admin.require_auth();
    assert!(new_admin != current_admin);

    storage::set_admin(&e, &new_admin);
}
