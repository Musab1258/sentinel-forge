use soroban_sdk::{Address, Env};

pub fn configure(e: Env, admin: Address, user: Address, score: i128) {
    admin.require_auth();

    if score > 0 {
        if score > 10 {
            registry::mark_priority(&e, &user);
        } else {
            registry::mark_standard(&e, &user);
        }
    } else {
        registry::mark_rejected(&e, &user);
    }
}
