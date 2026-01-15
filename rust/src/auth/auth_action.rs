use crate::pocketbase::{PbState, PocketBase};
use const_env::env_item;
use godot::classes::file_access::ModeFlags;
use godot::classes::FileAccess;
use pocketbase_sdk::client::Client;

#[env_item]
const AUTH_FILE: &str = "auth_token";
#[env_item]
pub const AUTH_PASS: &str = "default_pass";

pub struct AuthAction {}

impl AuthAction {
    pub fn auth(identifier: &str, secret: &str) -> Result<(), String> {
        let identifier = identifier.to_string();
        let secret = secret.to_string();

        let result = Self::auth_with_password(&identifier, &secret)?;
        let result = Self::save_auth_token(&result)?;

        Ok(())
    }

    fn auth_with_password(identifier: &str, secret: &str) -> Result<String, String> {
        let client = PocketBase::client();

        let no_auth_client = {
            let client = client.read().expect("Failed to read client");
            match &*client {
                PbState::NoAuth(client) => client.clone(),
                PbState::Auth(client) => Client::new(&client.base_url),
            }
        };

        let result = no_auth_client.auth_with_password("users", &identifier, &secret);
        if result.is_err() {
            return Err(result.unwrap_err().to_string());
        }

        let auth_client = result.unwrap();
        let token = auth_client.auth_token.clone().unwrap();

        let mut client = client.write().expect("Failed to write client");
        *client = PbState::Auth(auth_client);

        Ok(token)
    }

    fn save_auth_token(auth_token: &str) -> Result<(), String> {
        let Some(mut file) =
            FileAccess::open_encrypted_with_pass(AUTH_FILE, ModeFlags::WRITE, AUTH_PASS)
        else {
            return Err("Failed to open file".to_string());
        };

        if file.store_string(auth_token) {
            Ok(())
        } else {
            Err("Failed to save auth token".to_string())
        }
    }
}
