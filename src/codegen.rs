use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};

use anyhow::Result;
use dircpy::*;
use vina_story::content::{Character, Game, Scene};

pub fn generate_proj(game: &Game, output_dir: &Path) -> Result<()> {
    let mut project_path = output_dir.to_path_buf();
    project_path.push(game.name.clone());

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
    writeln!(file, r#"define config.name = _("{}")"#, game.name)?;
    writeln!(file, r#"define build.name = _("{}")"#, game.name)?;

    // TODO fill this in with the game's synopsis
    writeln!(
        file,
        r#"define gui.about = _p("""{}""")"#,
        "ai generated visual novel"
    )?;

    let script_path = project_path.join("game/script.rpy");
    let mut ctx = ScriptCtx::new(script_path);
    write_script(&mut ctx, &game)?;

    Ok(())
}

pub struct ScriptCtx {
    /// Indentation in number of tabs
    indent: usize,
    writer: BufWriter<File>,
}

impl ScriptCtx {
    pub fn new(output: PathBuf) -> Self {
        let file = OpenOptions::new().append(true).open(output).unwrap();

        Self {
            indent: 1,
            writer: BufWriter::new(file),
        }
    }

    pub fn indent_level(&self) -> usize {
        self.indent * 4
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn unindent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    pub fn write(&mut self, content: String) -> Result<()> {
        let indent = " ".repeat(self.indent_level());
        writeln!(self.writer, "{indent}{content}")?;
        Ok(())
    }
}

pub fn write_script(ctx: &mut ScriptCtx, game: &Game) -> Result<()> {
    // Character definitions
    for c in game.characters.iter() {
        ctx.write(format!(r#"define {} = Character("{}")"#, c.name, c.name))?;
    }

    ctx.write(format!("label start:"))?;
    ctx.indent();
    for (i, scene) in game.scenes.iter().enumerate() {
        gen_scene(ctx, game, &scene, i)?;
    }

    ctx.write(format!("return"))?;
    Ok(())
}

fn gen_scene(ctx: &mut ScriptCtx, game: &Game, scene: &Scene, i: usize) -> Result<()> {
    ctx.write(format!("label scene_{}:", i))?;

    ctx.indent();
    ctx.write(format!("scene bg bg_{}:", i))?;
    ctx.indent();

    ctx.write(format!("zoom 1.875"))?;
    ctx.unindent();

    for d in scene.script.iter() {
        if game
            .characters
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
            .contains(&d.speaker)
        {
            ctx.write(format!(
                r#"{} "{}""#,
                d.speaker,
                d.content.split(": ").last().unwrap_or(d.content.as_str())
            ))?;
        } else {
            ctx.write(format!(r#""{}""#, d.content))?;
        }
    }

    Ok(())
}
