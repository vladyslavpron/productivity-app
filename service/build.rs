use std::env;

pub fn main() {
    if Ok("release".to_owned()) == env::var("PROFILE") {
        println!("cargo:rustc-env=RUST_ENV=PRODUCTION");
    }
}
