use dotenvy::dotenv;
use vina_codegen::generate_proj;
fn main() {
    dotenv().ok();
    let ren_path: String = std::env::var("REN_PATH").unwrap_or("renpy".to_string());
    generate_proj(ren_path, "/Users/nithin/Renpy/".to_string(), "proj", vec![]).unwrap();
}
