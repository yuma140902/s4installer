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

pub fn install(_install_type: InstallType, _install_registry: InstallRegistry, _program: PathBuf) {
    todo!()
}

pub fn uninstall(_install_registry: InstallRegistry, _name: String) {
    todo!()
}

pub fn list(_install_registry: Option<InstallRegistry>) {
    todo!()
}
