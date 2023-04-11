use std::ffi::OsStr;

use anyhow::Result;
use common::file::download;
use xshell::{cmd, Shell};

use crate::config::Config;

pub const RELEASE_ARGS: [&str; 5] = [
    "-Z",
    "build-std=std,panic_abort",
    "-Z",
    "build-std-features=panic_immediate_abort",
    "--release",
];

pub const TEST_ARGS: [&str; 2] = ["-Z", "build-std=std"];

pub fn stage<I, II>(config: &Config<'_>, sh: &Shell, build_args: &I) -> Result<()>
where
    I: IntoIterator<Item = II> + Clone,
    II: AsRef<OsStr>,
{
    println!("Start: Staging cybercmd");
    let binary_path = if build_args
        .clone()
        .into_iter()
        .any(|item| item.as_ref() == "-r" || item.as_ref() == "--release")
    {
        &config.paths.release
    } else {
        &config.paths.debug
    };

    println!("Cleanup staging");
    config.paths.clean_staging()?;

    let cargo = &config.cargo_cmd;

    println!("Building cybercmd");
    {
        let pushed_dir = sh.push_dir(&config.paths.root);
        let build_iter = build_args.clone().into_iter();
        cmd!(sh, "{cargo} build {build_iter...} --package cybercmd").run()?;
        drop(pushed_dir);
    }

    println!("Copying cybercmd.dll to cybercmd.asi");
    sh.copy_file(
        binary_path.join("cybercmd.dll"),
        config.paths.staging_plugins.join("cybercmd.asi"),
    )?;

    println!("Done:  Staging cybercmd");

    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub fn stage_add_standalone(config: &Config<'_>) -> Result<()> {
    let global_ini = config.paths.staging_bin.join("global.ini");
    let version_dll = config.paths.staging_bin.join("version.dll");

    println!("Start: Staging standalone files");
    println!("Downloading global.ini");
    download(config.urls.global_ini, global_ini)?;
    println!("Downloading version.dll");
    download(config.urls.version_dll, version_dll)?;
    println!("Done:  Staging standalone files");

    Ok(())
}
