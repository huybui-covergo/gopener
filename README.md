# Gopener - Google Office Bridge

A cross-platform desktop application that allows you to upload Office files to Google Drive with automatic conversion to Google Workspace formats (Docs, Sheets, Slides).

## Features

- **Quick Upload**: Double-click any Office file to upload it to Google Drive
- **Drag & Drop**: Drag files into the app window for easy uploading
- **Automatic Conversion**: Files are automatically converted to Google Workspace format
- **Folder Selection**: Browse and select destination folders in Google Drive
- **Recent Files**: Quick access to recently uploaded files
- **Custom OAuth**: Use your own Google Cloud credentials for enhanced privacy

## Supported File Types

### Google Docs
- `.doc` - Microsoft Word 97-2003
- `.docx` - Microsoft Word 2007+
- `.odt` - OpenDocument Text
- `.rtf` - Rich Text Format
- `.txt` - Plain Text

### Google Sheets
- `.xls` - Microsoft Excel 97-2003
- `.xlsx` - Microsoft Excel 2007+
- `.ods` - OpenDocument Spreadsheet
- `.csv` - Comma Separated Values
- `.tsv` - Tab Separated Values

### Google Slides
- `.ppt` - Microsoft PowerPoint 97-2003
- `.pptx` - Microsoft PowerPoint 2007+
- `.odp` - OpenDocument Presentation

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/) 1.70+
- [Tauri CLI](https://tauri.app/) 2.x

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/gopener.git
   cd gopener
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Set up Google OAuth credentials:
   - Go to [Google Cloud Console](https://console.cloud.google.com/)
   - Create a new project or select an existing one
   - Enable the Google Drive API
   - Create OAuth 2.0 credentials (Desktop app)
   - Update the `DEFAULT_CLIENT_ID` in `src-tauri/src/commands/auth.rs`

4. Run in development mode:
   ```bash
   npm run tauri dev
   ```

### Building

Build for production:

```bash
npm run tauri build
```

This will create platform-specific installers in `src-tauri/target/release/bundle/`.

## Architecture

```
gopener/
├── src/                          # Vue 3 frontend
│   ├── components/               # Vue components
│   ├── stores/                   # Pinia state stores
│   ├── services/                 # API services
│   └── assets/                   # Styles and assets
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── commands/             # Tauri commands
│   │   ├── google/               # Google API integration
│   │   └── utils/                # Utilities
│   └── tauri.conf.json           # Tauri configuration
└── package.json
```

## Security

- OAuth tokens are stored securely in the system keychain
- PKCE flow is used for OAuth authentication
- Custom credentials are stored encrypted
- No sensitive data is transmitted to third-party servers

## License

MIT License - see [LICENSE](LICENSE) for details.
