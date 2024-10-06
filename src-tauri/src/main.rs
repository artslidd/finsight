// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use keyring::{credential, default, Entry, Error as KeyringError, Result};
use tauri_plugin_stronghold;

#[derive(serde::Serialize, Debug)]
struct UserInfo {
    username: String,
    auth_token: Option<String>,
}

enum LoginError {
    InvalidPassword,
    InvalidUsername,
    KeyringError,
}

fn get_token(username: &str) -> Option<String> {
    match Entry::new("finsight-user-auth-token", username) {
        Ok(entry) => match entry.get_password() {
            Ok(token) => Some(token),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

fn try_login(name: &str, password: &str) -> std::result::Result<UserInfo, LoginError> {
    if let Ok(entry_password) = Entry::new("finsight-user-password", name) {
        if let Ok(pw) = entry_password.get_password() {
            if pw == password {
                return Ok(UserInfo {
                    username: name.to_string(),
                    auth_token: get_token(name),
                });
            }
            return Err(LoginError::InvalidPassword);
        }
        return Err(LoginError::InvalidUsername);
    } else {
        Err(LoginError::KeyringError)
    }
}

fn set_entry(service: &str, name: &str, password: &str) -> Result<()> {
    let entry = Entry::new(service, name)?;
    entry.set_password(password)?;
    Ok(())
}

#[derive(serde::Serialize, Debug)]
enum LoginResult {
    LoggedIn(UserInfo),
    InvalidPassword,
    InvalidUsername,
    UnknownError,
}

#[tauri::command]
fn login(name: &str, password: &str) -> LoginResult {
    let r = match try_login(name, password) {
        Ok(info) => LoginResult::LoggedIn(info),
        Err(LoginError::InvalidPassword) => LoginResult::InvalidPassword,
        Err(LoginError::InvalidUsername) => LoginResult::InvalidUsername,
        Err(LoginError::KeyringError) => LoginResult::UnknownError,
    };
    println!("{:#?}", r);
    return r;
}

#[tauri::command]
fn register_user(name: &str, password: &str) -> bool {
    match set_entry("finsight-user-password", name, password) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
fn register_token(name: &str, token: &str) -> bool {
    match set_entry("finsight-user-auth-token", name, token) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            register_user,
            register_token,
            login
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
