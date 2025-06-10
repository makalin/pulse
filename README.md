# Pulse

**Pulse** is a secure, high-performance messaging app for phones and desktops, built with Rust for speed, safety, and privacy. Featuring end-to-end encryption, low-latency communication, and unique features like customizable themes and privacy-first AI, Pulse delivers a fast, reliable, and modern messaging experience.

## Features

- 🔒 **End-to-End Encryption**: Messages, calls, and media secured with AES-256 and the X3DH/Double Ratchet protocol.
- ⚡ **Ultra-Fast**: Built with Rust and QUIC for low-latency, real-time communication.
- 🌐 **Cross-Platform**: Native apps for iOS, Android, Windows, macOS, and Linux with seamless sync.
- 🎨 **Customizable UI**: Themes, animated stickers, and dynamic layouts for a personalized experience.
- 🤖 **Local AI Features**: On-device AI for smart replies, translations, and summaries, preserving privacy.
- 💬 **Group Chats**: Encrypted group chats (up to 1,000 users) with threaded replies.
- 🕒 **Self-Destructing Messages**: Auto-deleting messages for enhanced privacy.
- 📞 **High-Quality Calls**: Low-latency voice/video calls with noise suppression and background blur.

## Getting Started

### Prerequisites
- Rust >= 1.82 (with `cargo`)
- SQLite >= 3.45 (for local storage)
- OpenSSL >= 3.3 (for cryptographic operations)
- CMake and a C++ compiler (for native dependencies)
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
   cargo build --release --target aarch64-linux-android # For Android
   cargo build --release --target aarch64-apple-ios # For iOS
   ```
   - For Android: Use Android Studio to package the Rust binary into an APK.
   - For iOS: Use Xcode to bundle the Rust binary into an IPA.

4. **Desktop App Setup**
   ```bash
   cd desktop
   cargo build --release
   cargo run --release
   ```

5. **Configure Environment**
   - Initialize a SQLite database for local message storage.
   - Update `.env` files in `backend`, `mobile`, and `desktop` with server endpoints and encryption keys.

### Running Locally
- Start the backend: `cargo run --release --bin pulse-server`
- Run the mobile app: Use Android Studio/Xcode to deploy the compiled binary.
- Launch the desktop app: `cargo run --release --bin pulse-desktop`

## Project Structure
```
pulse/
├── backend/        # Rust-based QUIC server and API
├── mobile/         # Rust-based mobile app with native bindings
├── desktop/        # Rust-based desktop app with egui GUI
├── crypto/         # Rust cryptographic utilities (E2EE, key exchange)
├── docs/           # Documentation and protocol specs
└── scripts/        # Build and deployment scripts
```

## Contributing

We welcome contributions! Follow these steps:

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature/your-feature`.
3. Commit changes: `git commit -m "Add your feature"`.
4. Push to the branch: `git push origin feature/your-feature`.
5. Open a pull request.

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Security

Pulse prioritizes security:
- End-to-end encryption for all communications.
- Minimal, anonymized metadata storage.
- Rust’s memory safety prevents common vulnerabilities.
- Regular security audits.

## Roadmap
- [x] Core messaging with E2EE
- [x] Cross-platform sync
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
