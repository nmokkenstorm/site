use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    // here we forward build time environment variables to the compiled
    // binary as a macro so we can access a limited subset of the environment
    // in the compiled wasm binary
    for (key, value) in env::vars() {
        if (vec!["CONTENTFUL_SPACE_ID", "CONTENTFUL_DELIVERY_TOKEN"]).contains(&&*key) {
            println!("{}", format!("cargo:rustc-env={}={}", key, value));
        }
    }
}
