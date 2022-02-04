use std::path::PathBuf;

use clap::ArgEnum;
use clap::Parser;
use clap::Subcommand;

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
        /// copyはコピー、lnkはショートカット作成、symはシンボリックリンク作成、pwshはPowershellで対象のps1ファイルをいい感じに実行するように設定する。exeならlnk、ps1ならpwshを推奨
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

#[derive(ArgEnum, Debug, PartialEq, Clone, Copy)]
enum InstallType {
    Copy,
    Lnk,
    Sym,
    Pwsh,
}

#[derive(ArgEnum, Debug, PartialEq, Clone, Copy)]
#[clap(rename_all = "lower")]
enum InstallRegistry {
    Cli,
    SendTo,
}

fn main() {
    let arg: AppArg = AppArg::parse();
    dbg!(&arg);
}
