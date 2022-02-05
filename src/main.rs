use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

use s4installer::InstallRegistry;
use s4installer::InstallType;

#[derive(Parser, Debug)]
#[clap(
    name = "s4 installer",
    author,
    version,
    about = "An script and protable-executable installer for Windows"
)]
struct AppArg {
    #[clap(subcommand)]
    subcommand: AppSubCommand,
}

#[derive(Subcommand, Debug)]
enum AppSubCommand {
    /// Installs a program
    Install {
        #[clap(arg_enum, long = "type")]
        /// copyはコピー、lnkはショートカット作成、symはシンボリックリンク作成、pwshはPowershellで対象のps1ファイルをいい感じに実行するように設定する。sendtoかつexeならlnk、ps1ならpwshを推奨
        install_type: InstallType,

        #[clap(arg_enum, long = "for", value_name = "REGISTRY")]
        /// cliならパスを通すことでCLIから使えるようにする。sendtoならエクスプローラーの「送る」メニューに追加する。
        install_for: InstallRegistry,

        #[clap(value_name = "PROGRAM")]
        path_to_program: PathBuf,
    },
    /// Uninstalls a program installed by s4 installer
    Uninstall {
        #[clap(arg_enum, long, value_name = "REGISTRY")]
        from: InstallRegistry,

        program_name: String,
    },
    /// List installed programs
    List {
        #[clap(arg_enum, long = "in", value_name = "REGISTRY")]
        installed_in: Option<InstallRegistry>,
    },
}

fn main() {
    let arg: AppArg = AppArg::parse();
    dbg!(&arg);
    match arg.subcommand {
        AppSubCommand::Install {
            install_type,
            install_for,
            path_to_program,
        } => s4installer::install(install_type, install_for, path_to_program),
        AppSubCommand::Uninstall { from, program_name } => {
            s4installer::uninstall(from, program_name)
        }
        AppSubCommand::List { installed_in } => s4installer::list(installed_in),
    }
}
