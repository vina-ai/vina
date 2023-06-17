use std::{fs::OpenOptions, io::Write, time::SystemTime};

use anyhow::Result;
use dircpy::*;
pub fn generate_proj(
    ren_path: String,
    output_dir: String,
    project_name: &str,
    images: Vec<String>,
    description: String,
) -> Result<()> {
    println!("{}", ren_path);
    let project_path = output_dir + project_name;
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");

    copy_dir("./template/template", project_path.clone())?;
    let mut file = OpenOptions::new()
        .append(true)
        .open(project_path + "/game/options.rpy")?;
    writeln!(
        file,
        "define config.save_directory = \"{}-{}\"",
        "template",
        d.as_secs()
    )?;
    writeln!(file, "define config.name = _(\"{}\")", project_name)?;
    writeln!(file, "define build.name = _(\"{}\")", project_name)?;

    writeln!(file, "define gui.about = _p(\"\"\"{}\"\"\")", description)?;

    Ok(())
}
// define config.save_directory = "template-1686969152"
// define config.name = _("template")
// define gui.about = _p("""
// """)
// define build.name = "template"
