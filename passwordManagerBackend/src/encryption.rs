use aes_gcm::aead::Aead;
use aes_gcm::aead;
use aes_gcm::{Aes256Gcm, Key, aead::KeyInit, Nonce};
use argon2::Params;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};

pub fn derive_key(master_password: &str, salt: &str) -> [u8; 32] {
    let argon2 = Argon2::new(Default::default(), Default::default(), Params::default());
    let salt = SaltString::from_b64(salt).expect("invalid salt format");
    let hash = argon2.hash_password(master_password.as_bytes(), &salt)
        .expect("failed to hash password");

    let extracted_hash = hash.hash.expect("no hash found");
    let key_bytes = extracted_hash.as_bytes();
    let mut key = [0u8; 32];
    key.copy_from_slice(&key_bytes[..32]);
    key
}

pub fn encrypt_data(key: &[u8; 32], plaintext: &str) -> (String, String) {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let nonce = rand::random::<[u8; 12]>();
    let nonce_str = hex::encode(&nonce);
    let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), plaintext.as_bytes())
        .expect("encryption failed");
    let encrypted_hex = hex::encode(ciphertext);
    (nonce_str, encrypted_hex)
}

pub fn decrypt_data(key: &[u8; 32], nonce_hex: &str, encrypted_hex: &str) -> Result<String,aead::Error> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce= nonce_hex.trim_matches('"').to_string();
    let encrypted = encrypted_hex.trim_matches('"').to_string();


    let nonce_bytes = hex::decode(&nonce).expect("Invalid nonce hex");
    let ciphertext_bytes = hex::decode(encrypted).expect("Invalid encrypted_hex");

    let plaintext = match cipher.decrypt(Nonce::from_slice(&nonce_bytes), ciphertext_bytes.as_ref()){
        Ok(plaintext) => plaintext,
        Err(e) => return Err(e),
    };
    Ok(String::from_utf8(plaintext).expect("failed to decode decrypted text"))
}
