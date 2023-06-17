use std::{fs::OpenOptions, io::Write, path::PathBuf, time::SystemTime};

use anyhow::Result;
use dircpy::*;
pub fn generate_proj(
    ren_path: String,
    project_name: String,

    description: String,

    output_dir: std::path::PathBuf,

    data: Vec<String>,
) -> Result<()> {
    println!("{}", ren_path);
    let mut project_path = output_dir.clone();
    project_path.set_file_name(project_name.clone());

    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");

    copy_dir("./template/template", project_path.clone())?;
    let mut file = OpenOptions::new()
        .append(true)
        .open(project_path.join("/game/options.rpy"))?;
    writeln!(
        file,
        "define config.save_directory = \"{}-{}\"",
        "template",
        d.as_secs()
    )?;
    writeln!(file, "define config.name = _(\"{}\")", project_name.clone())?;
    writeln!(file, "define build.name = _(\"{}\")", project_name.clone())?;

    writeln!(file, "define gui.about = _p(\"\"\"{}\"\"\")", description)?;

    write_script(project_path, data);

    Ok(())
}
pub fn write_script(project_path: PathBuf, data: Vec<String>) {}
