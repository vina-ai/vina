use std::{
    collections::HashMap,
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

    copy_dir("./template/template", project_path.clone()).unwrap();
    copy_dir("./images", project_path.join("game/images")).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(project_path.join("game/options.rpy"))
        .unwrap();

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
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(output)
            .unwrap();

        Self {
            indent: 0,
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
    //Character definitions
    // for (i, c) in game.characters.iter().enumerate() {
    //     writeln!(file, r#"define {} = Character("{}")"#, c.name, c.name)?;
    //     writeln!(file, r#"image {}_img = "{}.png""#, c.name, c.name)?;
    // }

    // Character definitions
    for c in game.characters.iter() {
        ctx.write(format!(r#"define {} = Character("{}")"#, c.name, c.name))?;
        ctx.write(format!(r#"image {}_img = "{}.png""#, c.name, c.name))?;
        let expressions = vec!["smiling", "crying", "nervous", "excited", "blushing"];

        for emotion in expressions {
            ctx.write(format!(
                r#"image {}_img {} = "{}_{}.png""#,
                c.name, emotion, c.name, emotion
            ))?;
        }
    }

    ctx.write(format!("label start:"))?;
    for (i, scene) in game.scenes.iter().enumerate() {
        gen_scene(ctx, game, &scene, i)?;
    }

    ctx.write(format!("return"))?;
    Ok(())
}

fn gen_scene(ctx: &mut ScriptCtx, game: &Game, scene: &Scene, i: usize) -> Result<()> {
    let mut char_pos: HashMap<String, String> = HashMap::new();
    ctx.write(format!("label scene_{}:", i))?;
    ctx.indent();

    ctx.write(format!("scene bg bg_{}:", i))?;
    ctx.indent();
    ctx.write(format!("zoom 2"))?;
    ctx.unindent();

    for d in scene.script.iter() {
        if game
            .characters
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
            .contains(&d.speaker)
        {
            if !char_pos.contains_key(&d.speaker) {
                let position = match char_pos.len() {
                    1 => "left",
                    2 => "right",
                    _ => "center",
                };

                char_pos.insert(d.speaker.clone(), position.to_string());
            }
            ctx.write(format!(
                r#"{} "{}""#,
                d.speaker,
                d.content.split(": ").last().unwrap_or(d.content.as_str())
            ))?;
            ctx.write(format!(
                "show {}_img {} at {}",
                d.speaker, d.facial_expression, char_pos[&d.speaker]
            ))?;
        } else {
            ctx.write(format!(r#""{}""#, d.content))?;
        }
    }
    ctx.unindent();

    Ok(())
}
