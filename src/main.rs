use anyhow::{anyhow, Context, Result};
use std::env;
use std::env::{current_dir, set_current_dir};
use std::fs::{read_link, Metadata};
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let (this, script, args) = {
        let mut args = env::args();
        let this = args.next().context("program name is undefined")?;
        (
            this.clone(),
            args.next()
                .with_context(|| format!("Usage: {} <script> [ARG]...", this))?,
            args,
        )
    };

    match run(Path::new(&script), &args.collect::<Box<[_]>>()) {
        Ok(()) => {}

        Err(error) => {
            println!("{}: {}: {}", this, script, error);
        }
    }

    Ok(())
}

fn run(script_path: &Path, args: &[String]) -> Result<()> {
    let script_path = {
        let mut path = script_path.to_path_buf();
        while path.is_symlink() {
            path = read_link(path).context("read link")?;
        }
        path
    };
    if !script_path.is_file() {
        return Err(anyhow!("not a file or symlink"));
    };

    let script_dir = script_path
        .parent()
        .ok_or_else(|| anyhow!("no parent directory"))?;
    let bin_name = script_path.file_stem().expect("no filename");
    let bin_path = script_dir.join("target").join("release").join(bin_name);

    let needs_building = bin_path
        .metadata()
        .ok()
        .filter(Metadata::is_file)
        .as_ref()
        .map(Metadata::modified)
        .and_then(Result::ok)
        .map(|bin_modified| {
            WalkDir::new(script_dir.join("src"))
                .into_iter()
                .flat_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .flat_map(|entry| {
                    entry
                        .path()
                        .metadata()
                        .ok()
                        .as_ref()
                        .map(Metadata::modified)
                        .and_then(Result::ok)
                })
                .any(|modified| {
                    modified > bin_modified})
        })
        .unwrap_or(true);

    if needs_building {
        let script_current_dir = current_dir().context("get current directory")?;
        set_current_dir(script_dir.clone())
            .with_context(|| format!("set current directory to {}", script_dir.display()))?;

        let mut build_command = Command::new("cargo");
        build_command.args(["build", "--release"]);
        let anyhow_context = format!("execute build command: {:?}", &build_command);
        build_command
            .spawn()
            .context(anyhow_context.clone())?
            .wait()
            .context(anyhow_context)?;

        set_current_dir(script_current_dir.clone()).with_context(|| {
            format!("set current directory to {}", script_current_dir.display())
        })?;
    }

    let mut script_command = Command::new(bin_path);
    script_command.args(args);
    let anyhow_context = format!("execute script: {:?}", &script_command);
    script_command
        .spawn()
        .context(anyhow_context.clone())?
        .wait()
        .context(anyhow_context)?;

    Ok(())
}
