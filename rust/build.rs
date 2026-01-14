use dotenvy::dotenv;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=.env");

    match dotenv() {
        Ok(path) => {
            println!("cargo:warning=Environment loaded from: {}", path.display());

            for (key, value) in env::vars() {
                println!("cargo:rustc-env={}={}", key, value);
            }
        }
        Err(err) => {
            println!("cargo:warning=Failed to load environment: {}", err);
        }
    };
}
