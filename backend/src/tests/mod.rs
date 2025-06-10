#[cfg(test)]
mod crypto_tests {
    use pulse_crypto::Crypto;

    #[test]
    fn test_message_encryption() {
        let crypto = Crypto::new().unwrap();
        let message = b"Test message for Pulse";
        
        let encrypted = crypto.encrypt(message).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        
        assert_eq!(message, decrypted.as_slice());
    }
}

#[cfg(test)]
mod server_tests {
    use tokio;
    use tracing::{info, Level};
    use tracing_subscriber::FmtSubscriber;

    #[tokio::test]
    async fn test_server_initialization() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();

        info!("Testing server initialization...");
        // TODO: Add actual server initialization tests
    }
} 