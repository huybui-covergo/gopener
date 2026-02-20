# Gopener

A lightweight desktop app that uploads Office files to Google Drive and converts them to Google Workspace formats — Docs, Sheets, and Slides.

Built with [Tauri 2](https://tauri.app/), [Vue 3](https://vuejs.org/), and Rust.

## Features

- **Double-click to upload** — associate Office file types so they open directly in Gopener
- **Drag and drop** — drop files onto the window to upload
- **Auto-conversion** — files are converted to native Google Workspace formats on upload
- **Folder picker** — browse your Drive and choose a destination folder
- **Recent files** — quick access to your last 10 uploads
- **Custom OAuth credentials** — bring your own Google Cloud project for full control over API access
- **Secure storage** — tokens and credentials are stored in the system keychain (Keychain on macOS, Credential Manager on Windows, Secret Service on Linux)

## Supported Formats

| Target Format  | Source Extensions                            |
| -------------- | -------------------------------------------- |
| Google Docs    | `.doc`, `.docx`, `.odt`, `.rtf`, `.txt`      |
| Google Sheets  | `.xls`, `.xlsx`, `.ods`, `.csv`, `.tsv`      |
| Google Slides  | `.ppt`, `.pptx`, `.odp`                      |

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/) 1.70+
- Platform dependencies for Tauri — see the [Tauri prerequisites guide](https://v2.tauri.app/start/prerequisites/)

### Install and Run

```bash
git clone https://github.com/nicepkg/gopener.git
cd gopener
npm install
npm run tauri dev
```

### Google OAuth Setup

The app needs OAuth credentials to access Google Drive.

1. Go to the [Google Cloud Console](https://console.cloud.google.com/)
2. Create a project and enable the **Google Drive API**
3. Under **Credentials**, create an **OAuth 2.0 Client ID** (application type: Desktop)
4. Either:
   - Set the client ID in `src-tauri/src/commands/auth.rs` (`DEFAULT_CLIENT_ID`), or
   - Enter your credentials at runtime via **Settings > Custom OAuth**

Authentication uses the OAuth 2.0 PKCE flow — no client secret is required for the default flow.

### Build for Production

```bash
npm run tauri build
```

Installers are output to `src-tauri/target/release/bundle/`.

## Project Structure

```
src/                        # Frontend (Vue 3 + TypeScript)
├── components/             # UI — AuthButton, FileUploader, FolderBrowser, etc.
├── stores/                 # Pinia stores — auth, upload, settings, oauth-config
├── services/               # Tauri command wrappers and Google API helpers
└── assets/                 # Styles

src-tauri/                  # Backend (Rust)
├── src/
│   ├── commands/           # Tauri IPC commands — auth, upload, storage, file associations
│   ├── google/             # Drive API client and operations
│   └── utils/              # File type detection, keychain helpers
├── Cargo.toml
└── tauri.conf.json
```

## Scripts

| Command                    | Description                          |
| -------------------------- | ------------------------------------ |
| `npm run tauri dev`        | Start the app in development mode    |
| `npm run tauri build`      | Build platform-specific installers   |
| `npm run build`            | Build the frontend only              |
| `npm run test`             | Run frontend unit tests (Vitest)     |
| `npm run test:coverage`    | Run tests with coverage report       |

## CI

GitHub Actions runs on pull requests to `main`:

- **Frontend** — type checking (`vue-tsc`), unit tests, build
- **Backend** — Clippy lints and `cargo test` on Ubuntu, macOS, and Windows
- **Integration** — full Tauri build on all three platforms

## Security

- OAuth tokens stored in the OS keychain, not on disk
- PKCE flow — no client secret exposed in the binary
- Custom credentials encrypted before storage
- No telemetry or third-party data collection

## Platform Support

| Platform | Minimum Version |
| -------- | --------------- |
| Windows  | 10              |
| macOS    | 10.15 Catalina  |
| Linux    | WebKit2GTK 4.1+ |

## License

[MIT](LICENSE)
