use soroban_sdk::{Address, Env};

pub fn update_limit(e: Env, admin: Address, new_limit: i128) {
    admin.require_auth();
    assert!(new_limit > 0);

    let bounded_limit = new_limit.checked_add(1).unwrap();
    storage::set_limit(&e, bounded_limit);
}
