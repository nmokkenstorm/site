// in build.rs
fn main() {
    dotenv_build::output(dotenv_build::Config {
        filename: std::path::Path::new("./.env"),
        fail_if_missing_dotenv: true,
        ..Default::default()
    })
    .unwrap();
}
