// This module embeds the better-tor-cli.py script as a string and provides a function to write it to a temp file and return its path.
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub static PY_CLI_SCRIPT: &str = include_str!("../better-tor-cli.py");

pub fn write_embedded_cli_to_temp() -> std::io::Result<PathBuf> {
    let mut dir = env::temp_dir();
    dir.push("better-tor-cli.py");
    let mut file = File::create(&dir)?;
    file.write_all(PY_CLI_SCRIPT.as_bytes())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&dir, perms)?;
    }
    Ok(dir)
}
