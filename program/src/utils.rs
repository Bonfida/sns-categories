use {solana_program::hash::hashv, spl_name_service::state::HASH_PREFIX};

pub fn get_hashed_name(name: &str) -> Vec<u8> {
    hashv(&[(HASH_PREFIX.to_owned() + name).as_bytes()])
        .as_ref()
        .to_vec()
}
