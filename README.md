# Pulse

**Pulse** is a secure, high-performance messaging app for phones and desktops, built with Rust for speed, safety, and privacy. Featuring end-to-end encryption, low-latency communication, and unique features like customizable themes and privacy-first AI, Pulse delivers a fast, reliable, and modern messaging experience.

## Features

- 🔒 **End-to-End Encryption**: Messages secured with AES-256-GCM and X25519 key exchange.
- ⚡ **Ultra-Fast**: Built with Rust and QUIC for low-latency, real-time communication.
- 🌐 **Cross-Platform**: Native apps for iOS, Android, Windows, macOS, and Linux with seamless sync.
- 🎨 **Customizable UI**: Themes and dynamic layouts for a personalized experience.
- 💬 **Group Chats**: Encrypted group chats with message threading.
- 🕒 **Self-Destructing Messages**: Auto-deleting messages for enhanced privacy.
- 📱 **Native Mobile Experience**: Flutter-based mobile app with native Rust core.
- 🖥️ **Modern Desktop UI**: egui-based desktop client with responsive design.

## Getting Started

### Prerequisites
- Rust >= 1.82 (with `cargo`)
- SQLite >= 3.45 (for local storage)
- OpenSSL >= 3.3 (for cryptographic operations)
- Flutter SDK (for mobile development)
- Android NDK (for Android builds)
- Xcode (for iOS/macOS builds)

### Installation

1. **Clone the Repository**
   ```bash
   git clone https://github.com/makalin/pulse.git
   cd pulse
   ```

2. **Backend Setup**
   ```bash
   cd backend
   cargo build --release
   cp .env.example .env
   # Edit .env with SQLite path and QUIC server settings
   cargo run --release
   ```

3. **Mobile App Setup**
   ```bash
   cd mobile
   flutter pub get
   flutter build apk --release # For Android
   flutter build ios --release # For iOS
   ```

4. **Desktop App Setup**
   ```bash
   cd desktop
   cargo build --release
   cargo run --release
   ```

5. **Configure Environment**
   - Initialize a SQLite database for local message storage
   - Update `.env` files in `backend`, `mobile`, and `desktop` with server endpoints and encryption keys

### Running Locally
- Start the backend: `cargo run --release --bin pulse-server`
- Run the mobile app: Use Flutter to deploy the app
- Launch the desktop app: `cargo run --release --bin pulse-desktop`

## Project Structure
```
pulse/
├── backend/        # Rust-based QUIC server and API
│   ├── src/        # Server source code
│   │   ├── api.rs  # REST API endpoints
│   │   ├── db.rs   # Database operations
│   │   └── main.rs # Server entry point
├── mobile/         # Flutter-based mobile app with Rust core
│   ├── lib/        # Flutter UI code
│   └── src/        # Rust core functionality
├── desktop/        # Rust-based desktop app with egui GUI
│   ├── src/        # Desktop app source code
│   │   ├── ui/     # UI components
│   │   └── main.rs # Desktop app entry point
├── crypto/         # Rust cryptographic utilities
│   └── src/        # Encryption and key exchange
├── docs/           # Documentation and protocol specs
└── scripts/        # Build and deployment scripts
```

## Implemented Features

### Backend
- RESTful API endpoints for user management and messaging
- SQLite database integration for data persistence
- JWT-based authentication
- Message encryption and key management

### Desktop Client
- Modern egui-based user interface
- Multiple screens (Login, Chat, Settings)
- End-to-end encryption
- Local configuration management
- Message history
- Contact management
- Theme support
- Notification settings

### Mobile Client
- Flutter-based UI with native Rust core
- Local SQLite storage
- End-to-end encryption
- Message synchronization
- Chat management
- User authentication
- Configuration management
- Cross-platform support (iOS/Android)

### Crypto Module
- AES-256-GCM encryption
- X25519 key exchange
- Secure key storage
- Message signing and verification

## Contributing

We welcome contributions! Follow these steps:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Commit changes: `git commit -m "Add your feature"`
4. Push to the branch: `git push origin feature/your-feature`
5. Open a pull request

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Security

Pulse prioritizes security:
- End-to-end encryption for all communications
- Minimal, anonymized metadata storage
- Rust's memory safety prevents common vulnerabilities
- Regular security audits

## Roadmap
- [x] Core messaging with E2EE
- [x] Cross-platform sync
- [x] Basic UI implementation
- [ ] Voice and video calls (WebRTC integration)
- [ ] On-device AI features
- [ ] Public beta release

Check [ROADMAP.md](ROADMAP.md) for details.

## License

Pulse is licensed under the [AGPL-3.0 License](LICENSE). The codebase is open-source, and community contributions are encouraged.

## Contact

- **Website**: [pulseapp.dev](https://pulseapp.dev) (coming soon)
- **Email**: [support@pulseapp.dev](mailto:support@pulseapp.dev)
- **X**: [@PulseApp](https://x.com/PulseApp) (coming soon)

---

Built with 🦀 by the Pulse Team.
