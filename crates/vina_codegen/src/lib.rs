
use anyhow::Result;
use dircpy::*;
pub fn generate_proj(
    ren_path: String,
    output_dir: String,
    project_name: &str,
    images: Vec<String>,
) -> Result<()> {
    println!("{}", ren_path);
    copy_dir("./template/template", output_dir + project_name)?;
    Ok(())
}

