use {
    solana_program::hash::hashv,
    solana_program::pubkey::Pubkey,
    spl_name_service::state::{get_seeds_and_key, HASH_PREFIX},
};

pub fn get_hashed_name(name: &str) -> Vec<u8> {
    hashv(&[(HASH_PREFIX.to_owned() + name).as_bytes()])
        .as_ref()
        .to_vec()
}

pub fn get_category_metadata_key(category_name: &str) -> Pubkey {
    let hashed = get_hashed_name(category_name);
    let (key, _) = get_seeds_and_key(
        &spl_name_service::ID,
        hashed,
        Some(&crate::central_state::KEY),
        None,
    );
    key
}

pub fn get_category_member_key(category_member: &str, category: &Pubkey) -> Pubkey {
    let hashed = get_hashed_name(category_member);
    let (key, _) = get_seeds_and_key(&spl_name_service::ID, hashed, None, Some(category));
    key
}
