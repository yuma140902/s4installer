use std::fs;
use std::path::{Path, PathBuf};

use clap::ArgEnum;
use mslnk::{ShellLink, ShowCommand};
use path_absolutize::*;

#[derive(ArgEnum, Debug, PartialEq, Clone, Copy)]
pub enum InstallType {
    Copy,
    Lnk,
    Sym,
    Pwsh,
}

#[derive(ArgEnum, Debug, PartialEq, Clone, Copy)]
#[clap(rename_all = "lower")]
pub enum InstallRegistry {
    Cli,
    SendTo,
}

pub fn install(install_type: InstallType, install_registry: InstallRegistry, program: PathBuf) {
    let result = match install_registry {
        InstallRegistry::Cli => install_cli(install_type, &program),
        InstallRegistry::SendTo => install_sendto(install_type, &program),
    };
    if let Err(err) = result {
        eprintln!("Error during installing");
        eprintln!("{:?}", err);
    }
}

#[derive(Debug)]
enum InstallError {
    AccessTargetDir,
    NoProgram,
    AlreadyExists,
    FileIO(std::io::Error),
    NoPowerShellCore(which::Error),
}

fn install_cli(install_type: InstallType, program: &PathBuf) -> Result<(), InstallError> {
    if !program.is_file() {
        return Err(InstallError::NoProgram);
    }

    let s4dir = get_s4dir().ok_or_else(|| InstallError::AccessTargetDir)?;
    eprintln!("{} に手動でPATHを通してください", s4dir.to_string_lossy());

    match install_type {
        InstallType::Copy => install_copy(&s4dir, program),
        InstallType::Lnk => install_lnk(&s4dir, program),
        InstallType::Sym => install_symlink(&s4dir, program),
        InstallType::Pwsh => install_pwsh(&s4dir, program),
    }
}

fn install_copy(target_dir: &PathBuf, program: &PathBuf) -> Result<(), InstallError> {
    let dest = get_destination_file(target_dir, program).map_err(|_| InstallError::NoProgram)?;
    if dest.is_file() {
        return Err(InstallError::AlreadyExists);
    }

    eprintln!(
        "copying `{}` to `{}`",
        program.to_string_lossy(),
        dest.to_string_lossy()
    );
    fs::copy(program, dest).map_err(|err| InstallError::FileIO(err))?;

    Ok(())
}

fn install_lnk(
    target_dir: impl AsRef<Path>,
    program: impl AsRef<Path>,
) -> Result<(), InstallError> {
    let program = program
        .as_ref()
        .absolutize()
        .map_err(|err| InstallError::FileIO(err))?;

    let mut lnk = ShellLink::new(&program).map_err(|err| InstallError::FileIO(err))?;
    if let Some(program_dir) = program.parent() {
        lnk.set_working_dir(program_dir.to_str().map(|s| s.to_string()));
    }
    lnk.set_name(Some(format!(
        "Installed by s4 installer v{}",
        env!("CARGO_PKG_VERSION")
    )));

    let mut destination =
        get_destination_file(target_dir, &program).map_err(|_| InstallError::NoProgram)?;
    destination.set_extension("lnk");

    eprintln!("creating a shell link `{}`", destination.to_string_lossy());
    lnk.create_lnk(destination)
        .map_err(|err| InstallError::FileIO(err))?;

    Ok(())
}

fn install_symlink(
    target_dir: impl AsRef<Path>,
    program: impl AsRef<Path>,
) -> Result<(), InstallError> {
    let program = program
        .as_ref()
        .absolutize()
        .map_err(|err| InstallError::FileIO(err))?;

    let dest = get_destination_file(target_dir, &program).map_err(|_| InstallError::NoProgram)?;

    eprintln!(
        "creating symlink `{}` -> `{}`",
        dest.to_string_lossy(),
        program.to_string_lossy()
    );
    symlink::symlink_file(program, dest).map_err(|err| InstallError::FileIO(err))?;

    Ok(())
}

fn install_pwsh(
    target_dir: impl AsRef<Path>,
    program: impl AsRef<Path>,
) -> Result<(), InstallError> {
    let program = program
        .as_ref()
        .absolutize()
        .map_err(|err| InstallError::FileIO(err))?;

    let pwsh = which::which("pwsh").map_err(|err| InstallError::NoPowerShellCore(err))?;

    let mut lnk = ShellLink::new(&pwsh).map_err(|err| InstallError::FileIO(err))?;
    lnk.set_arguments(Some(format!("-noprofile {}", program.to_string_lossy())));
    lnk.header_mut()
        .set_show_command(ShowCommand::ShowMinNoActive);
    if let Some(program_dir) = program.parent() {
        lnk.set_working_dir(program_dir.to_str().map(|s| s.to_string()));
    }
    lnk.set_name(Some(format!(
        "Installed by s4 installer v{}",
        env!("CARGO_PKG_VERSION")
    )));

    let mut destination =
        get_destination_file(target_dir, &program).map_err(|_| InstallError::NoProgram)?;
    destination.set_extension("lnk");

    eprintln!("creating a shell link `{}`", destination.to_string_lossy());
    lnk.create_lnk(destination)
        .map_err(|err| InstallError::FileIO(err))?;

    Ok(())
}

fn install_sendto(_install_type: InstallType, _program: &PathBuf) -> Result<(), InstallError> {
    todo!()
}

fn get_s4dir() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    Some(home.join("s4\\scripts"))
}

fn get_destination_file<P: AsRef<Path>, Q: AsRef<Path>>(
    s4dir: P,
    program: Q,
) -> Result<PathBuf, ()> {
    let program_name: String;
    if let Some(file_name) = program.as_ref().file_name() {
        program_name = file_name.to_string_lossy().to_string();
    } else {
        return Err(());
    }
    Ok(s4dir.as_ref().join(program_name))
}

pub fn uninstall(_install_registry: InstallRegistry, _name: String) {
    todo!()
}

pub fn list(_install_registry: Option<InstallRegistry>) {
    todo!()
}
