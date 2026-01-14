use crate::auth::AuthAction;
use godot::classes::{Button, INode, LineEdit, Node};
use godot::global::{godot_error, godot_print};
use godot::obj::{Base, Gd, OnReady, WithBaseField};
use godot::register::{godot_api, GodotClass};
use godot_tokio::AsyncRuntime;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct AuthForm {
    base: Base<Node>,

    #[init(node = "%LoginInput")]
    login_input: OnReady<Gd<LineEdit>>,
    #[init(node = "%PasswordInput")]
    password_input: OnReady<Gd<LineEdit>>,
    #[init(node = "%SubmitButton")]
    submit_button: OnReady<Gd<Button>>,
}

#[godot_api]
impl INode for AuthForm {
    fn ready(&mut self) {
        let auth_callable = self.base().callable("auth");

        /*self.login_input.signals().text_submitted().connect(|_arg| {
            auth_callable.clone().call(&[]);
        });
        self.password_input
            .signals()
            .text_submitted()
            .connect(|_arg| {
                auth_callable.clone().call(&[]);
            });*/
        /*self.submit_button
        .signals()
        .pressed()
        .connect(auth_callable.bind(&[]));*/
        self.submit_button.connect("pressed", &auth_callable);
    }
}

#[godot_api]
impl AuthForm {
    #[func]
    fn auth(&self) {
        godot_print!("Auth button pressed");
        let identity = self.login_input.get_text();
        let secret = self.password_input.get_text();

        AsyncRuntime::block_on(async move {
            godot_print!("Authing...");
            match AuthAction::auth(&identity.to_string(), &secret.to_string()).await {
                Ok(_) => {
                    godot_print!("Auth successful");
                }
                Err(err) => {
                    godot_error!("Auth error: {}", err);
                }
            }
        });
    }
}
