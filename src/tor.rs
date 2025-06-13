use std::process::Command;
use std::path::PathBuf;

pub fn get_cli_path() -> PathBuf {
    PathBuf::from("/home/felipe/Documents/GitHub/better-ecosystem/better-tor/better-tor-cli.py")
}

pub fn check_tor_status(cli_path: &PathBuf) -> bool {
    let output = Command::new("python3")
        .arg(cli_path)
        .arg("--help")
        .output();
    match output {
        Ok(_) => {
            let iptables_output = Command::new("pkexec")
                .arg("iptables")
                .args(["-t", "nat", "-S"])
                .output();
            match iptables_output {
                Ok(output) => {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    output_str.contains("--to-ports 9040")
                },
                Err(_) => false
            }
        },
        Err(_) => false
    }
}

pub fn toggle_tor(cli_path: &PathBuf) -> Result<bool, String> {
    let output = Command::new("pkexec")
        .arg("python3")
        .arg(cli_path)
        .arg("--toggle")
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command failed: {}", stderr));
    }
    let new_status = check_tor_status(cli_path);
    Ok(new_status)
}
