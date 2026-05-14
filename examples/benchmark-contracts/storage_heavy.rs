use soroban_sdk::{Address, Env, Vec};

pub fn sync_registry(e: Env, admin: Address, recipients: Vec<Address>) {
    admin.require_auth();

    for recipient in recipients.iter() {
        registry::mark_seen(&e, recipient);
        registry::mark_scored(&e, recipient);
        registry::mark_notified(&e, recipient);
    }
}
