# Tauri Google Office Bridge - MVP Features & Implementation Guide

## Project Overview
A desktop application built with Tauri (Rust + Vue) that allows users to open local Office files and upload them to Google Workspace (Docs, Sheets, Slides) with automatic conversion.

---

## Supported File Extensions

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

---

## MVP Features (Minimum Viable Product)

### 1. Core Upload Functionality
**Priority: Critical**
- Drag-and-drop file upload
- File picker dialog for selecting files
- Automatic file type detection
- Upload to Google Drive with automatic conversion to Google format
- Real-time upload progress indicator
- Success/error notifications

### 2. Google Authentication
**Priority: Critical**
- OAuth 2.0 integration with Google
- Secure token storage in system keychain (via Tauri)
- Auto-refresh of access tokens
- Sign out functionality

### 3. Destination Folder Selection
**Priority: High**
- Browse Google Drive folders
- Select upload destination
- Remember last used folder (user preference)
- Quick access to "My Drive" root
- Create new folders on-the-fly

### 4. Basic Settings
**Priority: High**
- Choose default upload location
- Toggle "Open file after upload" option
- Toggle "Keep original file name" vs. custom naming
- Language selection (if supporting i18n)

### 5. OAuth Credential Options
**Priority: High**
- **Default App Credentials**: Use pre-configured OAuth credentials (easiest for most users)
- **Custom OAuth Credentials**: Allow users to provide their own Google Cloud OAuth credentials
  - Input fields for Client ID and Client Secret
  - Instructions/link to create Google Cloud project
  - Validation of credentials before saving
  - Secure storage of custom credentials in system keychain
- Benefits of custom credentials:
  - Full control over API quotas
  - Enhanced privacy (no third-party app access)
  - Required for enterprise/organizational accounts with restrictions

### 6. Recent Files History
**Priority: Medium**
- Display last 10-20 uploaded files
- Quick re-open in Google Docs/Sheets/Slides
- Clear history option

---

## Extended Features (Post-MVP)

### 7. Batch Upload
**Priority: Medium**
- Upload multiple files at once
- Queue management
- Pause/resume uploads

### 8. File Conversion Options
**Priority: Medium**
- Choose whether to convert to Google format or keep as Office format
- Preview before upload
- Format compatibility warnings

### 9. Advanced Organization
**Priority: Low**
- Auto-organize by file type into folders
- Add tags/labels during upload
- Custom naming templates (e.g., `{filename}_{date}`)

### 10. Offline Mode
**Priority: Low**
- Queue files for upload when connection restored
- Local cache of recent files metadata

### 11. Collaboration Features
**Priority: Low**
- Share link generation after upload
- Set permissions during upload
- Quick share to email addresses

### 12. System Integration
**Priority: Medium**
- Right-click context menu integration (Windows/macOS)
- Set as default app for Office files
- Global keyboard shortcuts

### 13. File Watching
**Priority: Low**
- Monitor specific folders for new Office files
- Auto-upload when detected
- Background service mode

---

## Technical Architecture

### Frontend (Vue 3)
```
src/
├── components/
│   ├── FileUploader.vue      # Main upload component
│   ├── QuickUpload.vue       # Double-click quick upload view
│   ├── FolderBrowser.vue     # Google Drive folder selector
│   ├── RecentFiles.vue       # History list
│   ├── Settings.vue          # User preferences
│   └── AuthButton.vue        # Google sign-in
├── stores/
│   ├── auth.ts               # Pinia store for authentication
│   ├── upload.ts             # Upload state management
│   ├── settings.ts           # User preferences
│   └── oauth-config.ts       # OAuth credential configuration
├── services/
│   ├── google-api.ts         # Google Drive API wrapper
│   └── tauri-commands.ts     # Tauri backend communication
└── App.vue
```

### Backend (Rust/Tauri)
```
src-tauri/
├── src/
│   ├── main.rs               # App entry point + CLI args handler
│   ├── commands/
│   │   ├── auth.rs           # OAuth handling
│   │   ├── upload.rs         # File upload logic
│   │   ├── storage.rs        # Secure credential storage
│   │   ├── fileassoc.rs      # File association registration
│   │   └── oauth_config.rs   # Custom OAuth credential management
│   ├── google/
│   │   ├── client.rs         # Google API client
│   │   └── drive.rs          # Drive-specific operations
│   └── utils/
│       ├── file.rs           # File type detection
│       └── keychain.rs       # System keychain integration
└── Cargo.toml
```

---

## MVP Implementation Steps

### Phase 1: Setup & Authentication (Week 1)
1. Initialize Tauri + Vue project
2. Set up Google OAuth 2.0 credentials
3. Implement authentication flow
4. Secure token storage using system keychain

### Phase 2: Core Upload (Week 2)
1. **File association registration** (critical for double-click)
2. **Command-line argument handling** (receive file path)
3. File picker integration
4. Drag-and-drop support
5. File type validation
6. Upload to Google Drive with conversion
7. Progress tracking

### Phase 3: Folder Management (Week 3)
1. Google Drive folder browser
2. Destination selection UI
3. Folder creation capability
4. Save user preferences

### Phase 4: Polish & Testing (Week 4)
1. Recent files history
2. Error handling and user feedback
3. UI/UX improvements
4. Testing across platforms (Windows, macOS, Linux)
5. Build and packaging

---

## API Requirements

### Google APIs Needed
- **Google Drive API v3**
  - `drive.file` scope (create and manage files)
  - `drive.readonly` scope (browse folders)
- **Google OAuth 2.0**
  - Desktop app flow

### Rust Crates
```toml
[dependencies]
tauri = "2.x"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "multipart"] }
keyring = "2.0"  # Secure credential storage
```

### npm Packages
```json
{
  "dependencies": {
    "vue": "^3.4",
    "pinia": "^2.1",
    "@tauri-apps/api": "^2.0",
    "axios": "^1.6",
    "@vueuse/core": "^10.7"
  }
}
```

---

## User Flow (MVP)

1. **First Launch**
   - User opens app
   - App shows credential choice:
     - "Use Default Credentials" (recommended for most users)
     - "Use My Own Credentials" (for advanced users/enterprises)
   - If custom credentials selected:
     - User enters Client ID and Client Secret
     - Link provided: "How to create Google OAuth credentials"
     - Credentials validated and stored securely
   - Clicks "Sign in with Google"
   - Authenticates via browser
   - Returns to app (authenticated)
   - App prompts: "Set as default app for Office files?" (optional)

2. **Upload File (Method 1: Double-Click)**
   - User double-clicks a .docx/.xlsx/.pptx file in File Explorer
   - App launches automatically with file pre-loaded
   - Quick upload dialog appears showing:
     - File name and type
     - Destination folder (with change option)
     - "Upload" button
   - User clicks "Upload" (or presses Enter)
   - Progress bar shows upload status
   - Success notification with "Open in Google" button
   - App can close automatically or stay open for more uploads

3. **Upload File (Method 2: Drag & Drop)**
   - User opens app
   - Drag file into app window OR click "Choose File"
   - App detects file type
   - User selects destination folder (or uses default)
   - User clicks "Upload"
   - Progress bar shows upload status
   - Success notification with "Open in Google" button

4. **Quick Re-upload**
   - User checks "Recent Files" list
   - Clicks on previous upload
   - Opens directly in Google Docs/Sheets/Slides

---

## Security Considerations

- Store OAuth tokens in system keychain (not plain text)
- Store custom OAuth credentials securely in system keychain
- Use PKCE flow for OAuth (required for both default and custom credentials)
- Validate file types before upload
- Implement file size limits
- Clear sensitive data on sign-out (including custom credentials if requested)
- Use HTTPS for all API calls
- Custom credentials never transmitted to app servers
- Validate custom Client ID format before storage

---

## Recommended MVP Scope

**Include in MVP:**
- Single file upload with conversion
- Double-click file association
- Command-line argument handling
- Google authentication with OAuth credential options (default or custom)
- Folder selection with browse capability
- Basic settings (default folder, auto-open, auto-close after upload)
- Recent files history (last 10)
- Drag-and-drop support

**Exclude from MVP (add later):**
- ❌ Batch uploads
- ❌ Advanced file organization
- ❌ System integration (context menus)
- ❌ Offline mode
- ❌ File watching

---

## Success Metrics for MVP

1. User can authenticate with Google in < 30 seconds
2. File uploads complete in < 5 seconds for typical documents
3. 95%+ success rate for supported file types
4. App launches in < 2 seconds
5. Zero crashes during normal operation

---

## Next Steps

1. Set up development environment
2. Create Google Cloud project and OAuth credentials
3. Initialize Tauri + Vue project structure
4. Begin Phase 1 implementation
5. Iterate based on user feedback
