use godot::classes::{Engine, IObject, Object};
use godot::obj::{Base, Gd, Singleton};
use godot::register::{GodotClass, godot_api};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct PocketBase {
    base: Base<Object>,

    client: Arc<RwLock<pocketbase_client::client::Client<pocketbase_client::client::NoAuth>>>,
}

#[godot_api]
impl IObject for PocketBase {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            client: Arc::new(RwLock::new(pocketbase_client::client::Client::new(""))),
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

    pub fn client()
    -> Arc<RwLock<pocketbase_client::client::Client<pocketbase_client::client::NoAuth>>> {
        match Self::singleton() {
            Some(singleton) => Arc::clone(&singleton.bind().client),
            None => panic!("PocketBase singleton not initialized"),
        }
    }
}
