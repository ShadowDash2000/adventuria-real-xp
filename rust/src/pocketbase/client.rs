use const_env::env_item;
use godot::classes::{Engine, IObject, Object};
use godot::obj::{Base, Gd, Singleton};
use godot::register::{godot_api, GodotClass};
use pocketbase_sdk::client::{Auth, Client, NoAuth};
use std::sync::{Arc, RwLock};

#[env_item]
const BASE_URL: &str = "http://127.0.0.1:8090";

pub enum PbState {
    NoAuth(Client<NoAuth>),
    Auth(Client<Auth>),
}

#[derive(GodotClass)]
#[class(base=Object)]
pub struct PocketBase {
    base: Base<Object>,

    client: Arc<RwLock<PbState>>,
}

#[godot_api]
impl IObject for PocketBase {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            client: Arc::new(RwLock::new(PbState::NoAuth(Client::new(BASE_URL)))),
        }
    }
}

#[godot_api]
impl PocketBase {
    pub const SINGLETON: &'static str = "PocketBase";

    pub fn singleton() -> Option<Gd<PocketBase>> {
        match Engine::singleton().get_singleton(Self::SINGLETON) {
            Some(singleton) => Some(singleton.cast::<Self>()),
            None => None,
        }
    }

    pub fn client() -> Arc<RwLock<PbState>> {
        match Self::singleton() {
            Some(singleton) => Arc::clone(&singleton.bind().client),
            None => panic!("PocketBase singleton not initialized"),
        }
    }

    pub fn no_auth_client() -> Option<Client<NoAuth>> {
        match &*Self::client().read().unwrap() {
            PbState::NoAuth(client) => Some(client.clone()),
            _ => None,
        }
    }

    pub fn auth_client() -> Option<Client<Auth>> {
        match &*Self::client().read().unwrap() {
            PbState::Auth(client) => Some(client.clone()),
            _ => None,
        }
    }
}
