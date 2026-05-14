use soroban_sdk::{Address, Env};

pub fn overwrite_recipient(e: Env, admin: Address, recipient: Address) {
    admin.require_auth();
    assert!(recipient != admin);

    storage::set_recipient(&e, &recipient);
}
