use std::path::Path;
use std::path::PathBuf;

use clap::ArgEnum;

use subcommands::install;
use subcommands::list;

mod subcommands;

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
        InstallRegistry::Cli => install::install_cli(install_type, &name, &program),
        InstallRegistry::SendTo => install::install_sendto(install_type, &name, &program),
    };
    if let Err(err) = result {
        eprintln!("Error during installing");
        eprintln!("{:?}", err);
    }
}

fn get_s4dir() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    Some(home.join("s4\\scripts"))
}

fn get_sendto_dir() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    Some(home.join(r"AppData\Roaming\Microsoft\Windows\SendTo"))
}

pub fn uninstall(_install_registry: InstallRegistry, _name: String) {
    todo!()
}

pub fn list(install_registry: Option<InstallRegistry>) {
    if install_registry.is_none() || install_registry == Some(InstallRegistry::Cli) {
        list::list_cli();
    }
    if install_registry.is_none() || install_registry == Some(InstallRegistry::SendTo) {
        list::list_sendto();
    }
}
