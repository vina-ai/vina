use std::{fs::OpenOptions, io::Write, path::PathBuf, time::SystemTime};

use anyhow::Result;
use dircpy::*;
use vina_story::content::{Character, Scene};
pub fn generate_proj(
    ren_path: String,
    project_name: String,

    description: String,

    output_dir: std::path::PathBuf,

    scenes: Vec<Scene>,
    characters: Vec<Character>,
) -> Result<()> {
    let mut project_path = output_dir.clone();
    project_path.push(project_name.clone());

    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");

    copy_dir("./template/template", project_path.clone())?;

    let mut file = OpenOptions::new()
        .append(true)
        .open(project_path.join("game/options.rpy"))?;

    writeln!(
        file,
        "define config.save_directory = \"{}-{}\"",
        "template",
        d.as_secs()
    )?;
    writeln!(
        file,
        r#"define config.name = _("{}")"#,
        project_name.clone()
    )?;
    writeln!(file, r#"define build.name = _("{}")"#, project_name.clone())?;

    writeln!(file, r#"define gui.about = _p("""{}""")"#, description)?;

    write_script(project_path, scenes, characters)?;

    Ok(())
}
pub fn write_script(
    project_path: PathBuf,
    scenes: Vec<Scene>,
    characters: Vec<Character>,
) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(project_path.join("game/script.rpy"))?;

    //Character definitions
    for c in characters {
        writeln!(file, r#"define {} = Character("{}")"#, c.name, c.name)?;
    }
    for (i, s) in scenes.into_iter().enumerate() {
        let mut indentation = 0;
        writeln!(file, "label scene_{}:", i)?;

        indentation += 4;

        for d in s.script {
            writeln!(
                file,
                "{}{} {}",
                " ".repeat(indentation),
                d.speaker,
                d.content
            )?;
        }
    }

    writeln!(file, "return")?;
    return Ok(());
}
