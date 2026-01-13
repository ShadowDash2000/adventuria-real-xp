use crate::interaction::Action;
use crate::pocketbase::PocketBase;
use godot::classes::Node;
use godot::obj::Base;
use godot::register::{godot_dyn, GodotClass};
use godot_tokio::AsyncRuntime;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct AuthAction {
    base: Base<Node>,
}

#[godot_dyn]
impl Action for AuthAction {
    fn interact(&mut self) {
        AsyncRuntime::spawn(async {
            let result = Self::auth_with_password("", "").await;
        });
    }
}

impl AuthAction {
    async fn auth_with_password(
        identifier: &str,
        secret: &str,
    ) -> Result<(), pocketbase_client::client::AuthError> {
        let client = PocketBase::client();

        let response = client
            .read()
            .await
            .auth_with_password("users", &identifier, &secret)
            .await?;

        client.write().await.auth_token = Some(response.auth_token.unwrap());
        Ok(())
    }
}
