use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Result, bail};
use pico_args::Arguments;

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    match args.subcommand()?.unwrap_or_default().as_str() {
        "dist" => {
            args.finish()?;

            fs::remove_dir_all(&dist_dir()).unwrap_or_else(|_| println!("dist dir doesn't exist"));
            fs::create_dir_all(&dist_dir())?;

            dist_binary()?;
        }
        unknown_script => {
            eprintln!("no such script exists: {}", unknown_script);
        }
    }
    Ok(())
}

fn dist_binary() -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    if !Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release"])
        .status()?
        .success()
    {
        bail!("cargo build failed");
    }

    let dst = project_root().join("target/release/backend");

    fs::copy(&dst, dist_dir().join("backend"))?;

    strip_binary(&dst)?;

    Ok(())
}

fn strip_binary(dst: &Path) -> Result<()> {
    if Command::new("strip").arg("--version").stdout(Stdio::null()).status().is_err() {
        eprintln!("no `strip` utility found");
        return Ok(());
    }

    println!("stripping the binary");

    if !Command::new("strip").arg(&dst).status()?.success() {
        bail!("strip failed");
    }

    Ok(())
}


fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).unwrap().to_path_buf()
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}
