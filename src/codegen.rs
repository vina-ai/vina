use std::{fs::OpenOptions, io::Write, path::PathBuf, time::SystemTime};

use anyhow::Result;
use dircpy::*;
use vina_story::content::{Character, Game, Scene};
pub fn generate_proj(
    ren_path: String,
    project_name: String,

    description: String,

    output_dir: std::path::PathBuf,
    game: Game,
) -> Result<()> {
    let mut project_path = output_dir.clone();
    project_path.push(project_name.clone());

    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");

    copy_dir("./template/template", project_path.clone())?;
    copy_dir("./images", project_path.join("game/images"))?;

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

    write_script(project_path, game)?;

    Ok(())
}
pub fn write_script(project_path: PathBuf, game: Game) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(project_path.join("game/script.rpy"))?;

    //Character definitions
    for c in game.characters.iter() {
        writeln!(file, r#"define {} = Character("{}")"#, c.name, c.name)?;
    }

    writeln!(file, "label start:")?;
    for (i, s) in game.scenes.into_iter().enumerate() {
        let mut indentation = 4;

        writeln!(file, "{}label scene_{}:", " ".repeat(indentation), i)?;

        indentation += 4;
        writeln!(file, "{}scene bg bg_{}:", " ".repeat(indentation), i)?;
        indentation += 4;

        writeln!(file, "{}zoom 1.875", " ".repeat(indentation))?;
        indentation -= 4;

        for d in s.script {
            if game
                .characters
                .iter()
                .map(|c| c.name.clone())
                .collect::<Vec<String>>()
                .contains(&d.speaker)
            {
                writeln!(
                    file,
                    r#"{}{} "{}""#,
                    " ".repeat(indentation),
                    d.speaker,
                    d.content.split(": ").last().unwrap_or(d.content.as_str())
                )?;
            } else {
                writeln!(file, r#"{} "{}""#, " ".repeat(indentation), d.content)?;
            }
        }
    }

    writeln!(file, "return")?;
    return Ok(());
}
