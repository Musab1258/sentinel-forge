use soroban_sdk::{Address, Env, Vec};

pub fn distribute(e: Env, admin: Address, recipients: Vec<Address>) {
    admin.require_auth();
    if recipients.len() == 0 {
        panic!("no recipients");
    }

    for recipient in recipients.iter() {
        storage::set_recipient(&e, recipient);
    }
}
