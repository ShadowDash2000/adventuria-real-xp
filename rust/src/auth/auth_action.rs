use crate::pocketbase::PocketBase;
use const_env::env_item;
use godot::classes::file_access::ModeFlags;
use godot::classes::FileAccess;

#[env_item]
const AUTH_FILE: &str = "auth_token";
#[env_item]
pub const AUTH_PASS: &str = "default_pass";

pub struct AuthAction {}

impl AuthAction {
    pub async fn auth(identifier: &str, secret: &str) -> Result<(), String> {
        let identifier = identifier.to_string();
        let secret = secret.to_string();

        let result = Self::auth_with_password(&identifier, &secret).await;
        if result.is_err() {
            return Err(result.unwrap_err().to_string());
        }

        let result = Self::save_auth_token(&result.unwrap());
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        Ok(())
    }

    async fn auth_with_password(
        identifier: &str,
        secret: &str,
    ) -> Result<String, pocketbase_client::client::AuthError> {
        let client = PocketBase::client();

        let response = client
            .read()
            .await
            .auth_with_password("users", &identifier, &secret)
            .await?;

        let auth_token = response.auth_token.unwrap();
        client.write().await.auth_token = Some(auth_token.clone());
        Ok(auth_token)
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
