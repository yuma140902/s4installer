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

pub fn install(
    install_type: InstallType,
    install_registry: InstallRegistry,
    name: &Option<String>,
    program: impl AsRef<Path>,
) {
    let result = match install_registry {
        InstallRegistry::Cli => install_cli(install_type, &name, &program),
        InstallRegistry::SendTo => install_sendto(install_type, &name, &program),
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

fn install_cli(
    install_type: InstallType,
    name: &Option<String>,
    program: impl AsRef<Path>,
) -> Result<(), InstallError> {
    if !program.as_ref().is_file() {
        return Err(InstallError::NoProgram);
    }

    let s4dir = get_s4dir().ok_or_else(|| InstallError::AccessTargetDir)?;
    eprintln!("{} に手動でPATHを通してください", s4dir.to_string_lossy());

    match install_type {
        InstallType::Copy => install_copy(&s4dir, name, program),
        InstallType::Lnk => install_lnk(&s4dir, name, program),
        InstallType::Sym => install_symlink(&s4dir, name, program),
        InstallType::Pwsh => install_pwsh(&s4dir, name, program),
    }
}

fn install_sendto(
    install_type: InstallType,
    name: &Option<String>,
    program: impl AsRef<Path>,
) -> Result<(), InstallError> {
    if !program.as_ref().is_file() {
        return Err(InstallError::NoProgram);
    }

    let sendto = get_sendto_dir().ok_or_else(|| InstallError::AccessTargetDir)?;

    match install_type {
        InstallType::Copy => install_copy(&sendto, name, program),
        InstallType::Lnk => install_lnk(&sendto, name, program),
        InstallType::Sym => install_symlink(&sendto, name, program),
        InstallType::Pwsh => install_pwsh(&sendto, name, program),
    }
}

fn install_copy(
    target_dir: impl AsRef<Path>,
    name: &Option<String>,
    program: impl AsRef<Path>,
) -> Result<(), InstallError> {
    let dest = get_destination_file(target_dir, name, &program, None)
        .map_err(|_| InstallError::NoProgram)?;
    if dest.is_file() {
        return Err(InstallError::AlreadyExists);
    }

    eprintln!(
        "copying `{}` to `{}`",
        program.as_ref().to_string_lossy(),
        dest.to_string_lossy()
    );
    fs::copy(program, dest).map_err(|err| InstallError::FileIO(err))?;

    Ok(())
}

fn install_lnk(
    target_dir: impl AsRef<Path>,
    name: &Option<String>,
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

    let destination = get_destination_file(target_dir, name, &program, Some("lnk"))
        .map_err(|_| InstallError::NoProgram)?;

    eprintln!("creating a shell link `{}`", destination.to_string_lossy());
    lnk.create_lnk(destination)
        .map_err(|err| InstallError::FileIO(err))?;

    Ok(())
}

fn install_symlink(
    target_dir: impl AsRef<Path>,
    name: &Option<String>,
    program: impl AsRef<Path>,
) -> Result<(), InstallError> {
    let program = program
        .as_ref()
        .absolutize()
        .map_err(|err| InstallError::FileIO(err))?;

    let dest = get_destination_file(target_dir, name, &program, None)
        .map_err(|_| InstallError::NoProgram)?;

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
    name: &Option<String>,
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

    let destination = get_destination_file(target_dir, name, &program, Some("lnk"))
        .map_err(|_| InstallError::NoProgram)?;

    eprintln!("creating a shell link `{}`", destination.to_string_lossy());
    lnk.create_lnk(destination)
        .map_err(|err| InstallError::FileIO(err))?;

    Ok(())
}

fn get_s4dir() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    Some(home.join("s4\\scripts"))
}

fn get_sendto_dir() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    Some(home.join(r"AppData\Roaming\Microsoft\Windows\SendTo"))
}

fn get_destination_file<P: AsRef<Path>, Q: AsRef<Path>>(
    s4dir: P,
    name: &Option<String>,
    program: Q,
    extension: Option<&str>,
) -> Result<PathBuf, InstallError> {
    let program_name = if let Some(name) = name {
        name.to_string()
    } else if let Some(file_name) = program.as_ref().file_name() {
        file_name.to_string_lossy().to_string()
    } else {
        return Err(InstallError::NoProgram);
    };
    let mut dest = s4dir.as_ref().join(program_name);
    if let Some(extension) = extension {
        dest.set_extension(extension);
    }
    Ok(dest)
}

pub fn uninstall(_install_registry: InstallRegistry, _name: String) {
    todo!()
}

pub fn list(_install_registry: Option<InstallRegistry>) {
    todo!()
}
