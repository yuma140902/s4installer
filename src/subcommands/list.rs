use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn show_programs(registry_name: &str, dir: impl AsRef<Path>) {
    let dir = dir.as_ref();
    let programs = get_installed_programs(dir);

    match programs {
        Ok(programs) => {
            eprintln!(
                "{} programs installed in {} ({})",
                programs.len(),
                registry_name,
                dir.to_string_lossy().to_string()
            );
            for program in programs {
                eprintln!(
                    "    {}",
                    program
                        .file_name()
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_default()
                );
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err)
        }
    }
}

pub fn list_cli() {
    let s4dir = crate::get_s4dir();
    if let Some(dir) = s4dir {
        show_programs("CLI", dir);
    } else {
        eprintln!("Error: s4dir not found");
    }
}

pub fn list_sendto() {
    let sendto_dir = crate::get_sendto_dir();
    if let Some(dir) = sendto_dir {
        show_programs("SendTo", dir);
    } else {
        eprintln!("Error: SendTo dir not found");
    }
}

fn get_installed_programs(dir: impl AsRef<Path>) -> Result<Vec<PathBuf>, std::io::Error> {
    let program_exts = {
        let mut p = get_program_exts();
        p.push("ps1".to_string());
        p.push("lnk".to_string());
        p
    };

    let mut programs = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if is_program_file(&path, &program_exts) {
            programs.push(path);
        }
    }
    Ok(programs)
}

fn get_program_exts() -> Vec<String> {
    let pathext = std::env::var("PATHEXT");
    if let Ok(pathext) = pathext {
        pathext
            .split(';')
            .map(|s| s.trim_start_matches('.').to_ascii_lowercase())
            .collect()
    } else {
        Vec::with_capacity(0)
    }
}

fn is_program_file(file: impl AsRef<Path>, program_exts: &Vec<String>) -> bool {
    let file = file.as_ref();
    let file = if file.is_symlink() {
        if let Ok(f) = file.read_link() {
            f
        } else {
            return false;
        }
    } else {
        file.to_path_buf()
    };

    if let Some(ext) = file.extension() {
        program_exts.contains(&ext.to_string_lossy().to_ascii_lowercase())
    } else {
        false
    }
}
