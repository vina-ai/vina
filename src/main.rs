use clap::Parser;
use dotenvy::dotenv;
use vina_codegen::generate_proj;

#[derive(Parser)]
struct Cli {
    name: String,
    prompt: String,
    out: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    dotenv().ok();

    let ren_path: String = std::env::var("REN_PATH").unwrap_or("renpy".to_string());
    let openai_token = std::env::var("OPENAI_KEY").expect("Could not get OPENAI_KEY");

    generate_proj(ren_path, args.name, args.prompt, args.out, vec![]).unwrap();
}
