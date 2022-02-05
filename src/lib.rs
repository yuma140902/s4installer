use std::fs;
use std::path::PathBuf;

use clap::ArgEnum;

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
    AccessS4Dir,
    NoProgram,
    AlreadyExists,
    FileIO(std::io::Error),
}

fn install_cli(install_type: InstallType, program: &PathBuf) -> Result<(), InstallError> {
    if !program.is_file() {
        return Err(InstallError::NoProgram);
    }

    let s4dir = get_s4dir().ok_or_else(|| InstallError::AccessS4Dir)?;
    eprintln!("{} に手動でPATHを通してください", s4dir.to_string_lossy());

    match install_type {
        InstallType::Copy => install_cli_copy(&s4dir, program),
        InstallType::Lnk => todo!(),
        InstallType::Sym => todo!(),
        InstallType::Pwsh => todo!(),
    }
}

fn install_cli_copy(s4dir: &PathBuf, program: &PathBuf) -> Result<(), InstallError> {
    let program_name: String;
    if let Some(file_name) = program.file_name() {
        program_name = file_name.to_string_lossy().to_string();
    } else {
        return Err(InstallError::NoProgram);
    }
    let dest = s4dir.join(program_name);
    if dest.is_file() {
        return Err(InstallError::AlreadyExists);
    }

    eprintln!(
        "copying `{}` to `{}`",
        program.to_string_lossy(),
        dest.to_string_lossy()
    );
    fs::copy(program, dest)
        .map_err(|err| InstallError::FileIO(err))
        .map(|_| ())
}

fn install_sendto(_install_type: InstallType, _program: &PathBuf) -> Result<(), InstallError> {
    todo!()
}

fn get_s4dir() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    Some(home.join("s4\\scripts"))
}

pub fn uninstall(_install_registry: InstallRegistry, _name: String) {
    todo!()
}

pub fn list(_install_registry: Option<InstallRegistry>) {
    todo!()
}
