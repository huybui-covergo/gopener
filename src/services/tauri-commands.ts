import { invoke } from "@tauri-apps/api/core";

// Auth commands
export async function getAuthUrl(): Promise<string> {
  return invoke("get_auth_url");
}

export async function exchangeCode(code: string): Promise<{
  is_authenticated: boolean;
  access_token: string | null;
  expires_at: number | null;
}> {
  return invoke("exchange_code", { code });
}

export async function refreshToken(): Promise<{
  is_authenticated: boolean;
  access_token: string | null;
  expires_at: number | null;
}> {
  return invoke("refresh_token");
}

export async function signOut(): Promise<void> {
  return invoke("sign_out");
}

export async function checkAuth(): Promise<{
  is_authenticated: boolean;
  access_token: string | null;
  expires_at: number | null;
}> {
  return invoke("check_auth");
}

// Upload commands
export interface UploadResult {
  file_id: string;
  name: string;
  web_view_link: string;
  file_type: string;
}

export async function uploadFile(
  filePath: string,
  folderId?: string | null
): Promise<UploadResult> {
  return invoke("upload_file", { filePath, folderId });
}

// Storage commands
export interface Settings {
  default_folder_id: string | null;
  default_folder_name: string | null;
  auto_open_after_upload: boolean;
  auto_close_after_upload: boolean;
  theme: string;
  recent_files: RecentFile[];
}

export interface RecentFile {
  id: string;
  name: string;
  path: string;
  google_url: string;
  file_type: string;
  uploaded_at: number;
}

export async function getSettings(): Promise<Settings> {
  return invoke("get_settings");
}

export async function saveSettings(settings: Settings): Promise<void> {
  return invoke("save_settings", { settings });
}

export async function addRecentFile(file: RecentFile): Promise<void> {
  return invoke("add_recent_file", { file });
}

export async function clearRecentFiles(): Promise<void> {
  return invoke("clear_recent_files");
}

// OAuth config commands
export interface OAuthConfig {
  use_custom: boolean;
  client_id: string | null;
  has_client_secret: boolean;
}

export async function getOAuthConfig(): Promise<OAuthConfig> {
  return invoke("get_oauth_config");
}

export async function saveOAuthConfig(
  clientId: string,
  clientSecret: string
): Promise<void> {
  return invoke("save_oauth_config", { clientId, clientSecret });
}

export async function clearOAuthConfig(): Promise<void> {
  return invoke("clear_oauth_config");
}

// File association commands
export async function registerFileAssociations(): Promise<string> {
  return invoke("register_file_associations");
}

// Google Drive commands
export interface DriveFolder {
  id: string;
  name: string;
  mime_type: string;
}

export async function listFolders(
  parentId?: string | null
): Promise<DriveFolder[]> {
  return invoke("list_folders", { parentId });
}

export async function createFolder(
  name: string,
  parentId?: string | null
): Promise<DriveFolder> {
  return invoke("create_folder", { name, parentId });
}
