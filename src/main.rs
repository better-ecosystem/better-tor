use std::process::Command;
use std::path::PathBuf;
use std::env;

use gtk4::prelude::*;
use gtk4::{Application, Button, Box, Orientation};
use libadwaita::prelude::*;
use libadwaita::{ApplicationWindow as AdwApplicationWindow, HeaderBar, StatusPage, Toast, ToastOverlay};

const APP_ID: &str = "com.betterecosystem.tor";

#[tokio::main]
async fn main() {
    // Initialize libadwaita
    libadwaita::init().unwrap();

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // Get the path to the CLI script
    let cli_path = get_cli_path();
    
    // Create main window
    let window = AdwApplicationWindow::builder()
        .application(app)
        .default_width(300)
        .default_height(400)
        .resizable(false)
        .title("Better Tor")
        .build();

    // Create toast overlay for notifications
    let toast_overlay = ToastOverlay::new();

    // Create main content box
    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(20)
        .margin_top(40)
        .margin_bottom(40)
        .margin_start(40)
        .margin_end(40)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .build();

    // Create status page with power icon (currently unused but may be useful later)
    let _status_page = StatusPage::builder()
        .icon_name("system-shutdown-symbolic")
        .title("Tor Anonymizer")
        .description("Click the power button to toggle Tor routing")
        .build();

    // Create power button
    let power_button = Button::builder()
        .icon_name("system-shutdown-symbolic")
        .width_request(120)
        .height_request(120)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .css_classes(vec!["circular", "suggested-action"])
        .build();

    // Create status label
    let status_label = gtk4::Label::builder()
        .label("Status: Unknown")
        .css_classes(vec!["title-2"])
        .halign(gtk4::Align::Center)
        .build();

    // Check initial status
    let initial_status = check_tor_status(&cli_path);
    update_ui_for_status(&power_button, &status_label, initial_status);

    // Handle power button click
    let cli_path_clone = cli_path.clone();
    let power_button_clone = power_button.clone();
    let status_label_clone = status_label.clone();
    let toast_overlay_clone = toast_overlay.clone();
    
    power_button.connect_clicked(move |_button| {
        let cli_path = cli_path_clone.clone();
        let button = power_button_clone.clone();
        let status_label = status_label_clone.clone();
        let toast_overlay = toast_overlay_clone.clone();
        
        // Disable button during operation
        button.set_sensitive(false);
        button.set_icon_name("process-working-symbolic");
        
        // Execute command in background
        gtk4::glib::spawn_future_local(async move {
            let cli_path_for_task = cli_path.clone();
            let result = tokio::task::spawn_blocking(move || {
                toggle_tor(&cli_path_for_task)
            }).await;
            
            match result {
                Ok(Ok(new_status)) => {
                    update_ui_for_status(&button, &status_label, new_status);
                    let message = if new_status {
                        "Tor anonymizer is now ON"
                    } else {
                        "Tor anonymizer is now OFF"
                    };
                    let toast = Toast::new(message);
                    toast_overlay.add_toast(toast);
                },
                Ok(Err(error)) => {
                    // Reset button state on error
                    let current_status = check_tor_status(&cli_path);
                    update_ui_for_status(&button, &status_label, current_status);
                    
                    // Print the full error to the terminal for debugging
                    eprintln!("[Better Tor GUI] Error: {}", error);
                    
                    // Show a shortened error in the toast
                    let toast = Toast::new(&format!("Error: {}", error.chars().take(100).collect::<String>()));
                    toast_overlay.add_toast(toast);
                },
                Err(_) => {
                    // Reset button state on panic
                    let current_status = check_tor_status(&cli_path);
                    update_ui_for_status(&button, &status_label, current_status);
                    
                    let toast = Toast::new("An unexpected error occurred");
                    toast_overlay.add_toast(toast);
                }
            }
            
            button.set_sensitive(true);
        });
    });

    // Add widgets to content box
    content_box.append(&power_button);
    content_box.append(&status_label);

    // Set up window structure
    toast_overlay.set_child(Some(&content_box));
    window.set_content(Some(&toast_overlay));

    // Show window
    window.present();
}

fn get_cli_path() -> PathBuf {
    // Always use the absolute path to the CLI script in the project directory
    PathBuf::from("/home/felipe/Documents/GitHub/better-ecosystem/better-tor/better-tor-cli.py")
}

fn check_tor_status(cli_path: &PathBuf) -> bool {
    // Run the CLI script to check if Tor is currently active
    // We'll use a simple approach by checking if the iptables rules are loaded
    let output = Command::new("python3")
        .arg(cli_path)
        .arg("--help") // Just to check if the script is accessible
        .output();
    
    match output {
        Ok(_) => {
            // Check if iptables rules are loaded by running iptables command
            let iptables_output = Command::new("iptables")
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

fn toggle_tor(cli_path: &PathBuf) -> Result<bool, String> {
    // Check current status first
    let current_status = check_tor_status(cli_path);
    
    // Execute the toggle command
    let output = Command::new("pkexec") // Use pkexec for GUI privilege escalation
        .arg("python3")
        .arg(cli_path)
        .arg("--toggle")
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command failed: {}", stderr));
    }
    
    // Return the new status (opposite of current)
    Ok(!current_status)
}

fn update_ui_for_status(button: &Button, status_label: &gtk4::Label, is_active: bool) {
    if is_active {
        button.set_icon_name("system-shutdown-symbolic");
        button.remove_css_class("suggested-action");
        button.add_css_class("destructive-action");
        status_label.set_text("Status: ON");
        status_label.remove_css_class("error");
        status_label.add_css_class("success");
    } else {
        button.set_icon_name("system-shutdown-symbolic");
        button.remove_css_class("destructive-action");
        button.add_css_class("suggested-action");
        status_label.set_text("Status: OFF");
        status_label.remove_css_class("success");
        status_label.add_css_class("error");
    }
}
