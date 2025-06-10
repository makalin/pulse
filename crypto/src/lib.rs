use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    #[error("Decryption failed: {0}")]
    DecryptionError(String),
    #[error("Key generation failed: {0}")]
    KeyGenerationError(String),
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),
    #[error("Invalid message format: {0}")]
    InvalidMessageFormat(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    public_key: PublicKey,
    private_key: EphemeralSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
    associated_data: Option<Vec<u8>>,
}

pub struct Crypto {
    key: Key<Aes256Gcm>,
    key_pair: Option<KeyPair>,
}

impl Crypto {
    pub fn new() -> Result<Self, CryptoError> {
        let mut key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut key_bytes);
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        Ok(Self { 
            key: key.clone(),
            key_pair: None,
        })
    }

    pub fn generate_key_pair(&mut self) -> Result<PublicKey, CryptoError> {
        let private_key = EphemeralSecret::random_from_rng(&mut OsRng);
        let public_key = PublicKey::from(&private_key);
        
        self.key_pair = Some(KeyPair {
            public_key: public_key.clone(),
            private_key,
        });

        Ok(public_key)
    }

    pub fn encrypt(&self, data: &[u8], associated_data: Option<&[u8]>) -> Result<EncryptedMessage, CryptoError> {
        let cipher = Aes256Gcm::new(&self.key);
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = match associated_data {
            Some(ad) => cipher
                .encrypt(nonce, data)
                .map_err(|e| CryptoError::EncryptionError(e.to_string()))?,
            None => cipher
                .encrypt(nonce, data)
                .map_err(|e| CryptoError::EncryptionError(e.to_string()))?,
        };

        Ok(EncryptedMessage {
            nonce: nonce_bytes.to_vec(),
            ciphertext,
            associated_data: associated_data.map(|ad| ad.to_vec()),
        })
    }

    pub fn decrypt(&self, message: &EncryptedMessage) -> Result<Vec<u8>, CryptoError> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Nonce::from_slice(&message.nonce);

        cipher
            .decrypt(nonce, message.ciphertext.as_slice())
            .map_err(|e| CryptoError::DecryptionError(e.to_string()))
    }

    pub fn derive_shared_secret(&self, peer_public_key: &PublicKey) -> Result<SharedSecret, CryptoError> {
        let key_pair = self.key_pair.as_ref()
            .ok_or_else(|| CryptoError::KeyGenerationError("No key pair available".to_string()))?;
        
        Ok(key_pair.private_key.diffie_hellman(peer_public_key))
    }

    pub fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // TODO: Implement message signing using Ed25519
        Err(CryptoError::KeyGenerationError("Not implemented".to_string()))
    }

    pub fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<bool, CryptoError> {
        // TODO: Implement signature verification using Ed25519
        Err(CryptoError::KeyGenerationError("Not implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let crypto = Crypto::new().unwrap();
        let original_data = b"Hello, Pulse!";
        
        let encrypted = crypto.encrypt(original_data, None).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        
        assert_eq!(original_data, decrypted.as_slice());
    }

    #[test]
    fn test_key_pair_generation() {
        let mut crypto = Crypto::new().unwrap();
        let public_key = crypto.generate_key_pair().unwrap();
        assert!(crypto.key_pair.is_some());
    }

    #[test]
    fn test_encryption_with_associated_data() {
        let crypto = Crypto::new().unwrap();
        let message = b"Secret message";
        let associated_data = b"Metadata";
        
        let encrypted = crypto.encrypt(message, Some(associated_data)).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        
        assert_eq!(message, decrypted.as_slice());
    }
} 