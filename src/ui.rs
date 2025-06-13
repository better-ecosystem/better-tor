use crate::network::get_ip_info;
use crate::tor::{check_tor_status, get_cli_path, toggle_tor};
use gtk4::prelude::*;
use gtk4::{Application, Box, Button, Orientation, Spinner};
use libadwaita::prelude::*;
use libadwaita::{ApplicationWindow as AdwApplicationWindow, StatusPage, Toast, ToastOverlay};
use std::rc::Rc;

pub fn build_ui(app: &Application) {
    let cli_path = get_cli_path();
    let window = AdwApplicationWindow::builder()
        .application(app)
        .default_width(300)
        .default_height(400)
        .resizable(false)
        .title("Better Tor")
        .build();
    let toast_overlay = ToastOverlay::new();
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
    let _status_page = StatusPage::builder()
        .icon_name("system-shutdown-symbolic")
        .title("Tor Anonymizer")
        .description("Click the power button to toggle Tor routing")
        .build();
    let power_button = Button::builder()
        .icon_name("system-shutdown-symbolic")
        .width_request(120)
        .height_request(120)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .css_classes(vec!["circular", "suggested-action"])
        .build();
    let status_label = gtk4::Label::builder()
        .label("Status: Unknown")
        .css_classes(vec!["title-2"])
        .halign(gtk4::Align::Center)
        .build();
    let spinner = Spinner::new();
    spinner.set_halign(gtk4::Align::Center);
    spinner.set_valign(gtk4::Align::Center);
    spinner.set_size_request(24, 24);
    spinner.set_visible(true);
    spinner.start();
    let ip_label = Rc::new(
        gtk4::Label::builder()
            .label("")
            .css_classes(vec!["title-3"])
            .halign(gtk4::Align::Center)
            .build(),
    );
    let country_label = Rc::new(
        gtk4::Label::builder()
            .label("")
            .css_classes(vec!["title-4"])
            .halign(gtk4::Align::Center)
            .build(),
    );
    let ip_info_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .halign(gtk4::Align::Center)
        .build();
    let ip_box = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(gtk4::Align::Center)
        .build();
    ip_box.append(&spinner);
    ip_box.append(ip_label.as_ref());
    ip_info_box.append(&ip_box);
    ip_info_box.append(country_label.as_ref());
    let ip_label_clone = ip_label.clone();
    let country_label_clone = country_label.clone();
    let spinner_clone = spinner.clone();
    glib::MainContext::default().spawn_local(async move {
        let ip_info = get_ip_info().await;
        spinner_clone.stop();
        spinner_clone.set_visible(false);
        ip_label_clone.set_text(&ip_info.ip);
        country_label_clone.set_text(&ip_info.country);
    });
    let initial_status = check_tor_status(&cli_path);
    update_ui_for_status(&power_button, &status_label, initial_status);
    let cli_path_clone = cli_path.clone();
    let power_button_clone = power_button.clone();
    let status_label_clone = status_label.clone();
    let toast_overlay_clone = toast_overlay.clone();
    let ip_label_for_closure = ip_label.clone();
    let country_label_for_closure = country_label.clone();
    let power_spinner = Rc::new(Spinner::new());
    power_spinner.set_halign(gtk4::Align::Center);
    power_spinner.set_valign(gtk4::Align::Center);
    power_spinner.set_size_request(32, 32);
    power_spinner.set_visible(false);
    let power_icon = Rc::new(gtk4::Image::from_icon_name("system-shutdown-symbolic"));
    power_icon.set_pixel_size(96);
    power_button.set_child(Some(power_icon.as_ref()));
    content_box.append(&ip_info_box);
    content_box.append(&power_button);
    content_box.append(&status_label);
    toast_overlay.set_child(Some(&content_box));
    window.set_content(Some(&toast_overlay));
    window.present();
    let power_spinner_for_closure = power_spinner.clone();
    let power_icon_for_closure = power_icon.clone();
    let power_button_rc = Rc::new(power_button);
    let power_button_for_closure = power_button_rc.clone();
    power_button_rc.connect_clicked(move |_button| {
        let cli_path = cli_path_clone.clone();
        let button = power_button_clone.clone();
        let status_label = status_label_clone.clone();
        let toast_overlay = toast_overlay_clone.clone();
        let ip_label = ip_label_for_closure.clone();
        let country_label = country_label_for_closure.clone();
        button.set_sensitive(false);
        power_icon_for_closure.set_visible(false);
        power_button_for_closure.set_child(Some(power_spinner_for_closure.as_ref()));
        power_spinner_for_closure.set_visible(true);
        power_spinner_for_closure.start();
        let power_spinner_inner = power_spinner_for_closure.clone();
        let power_icon_inner = power_icon_for_closure.clone();
        let power_button_inner = power_button_for_closure.clone();
        glib::MainContext::default().spawn_local(async move {
            let cli_path_for_task = cli_path.clone();
            let result = tokio::task::spawn_blocking(move || toggle_tor(&cli_path_for_task)).await;
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
                }
                Ok(Err(error)) => {
                    let current_status = check_tor_status(&cli_path);
                    update_ui_for_status(&button, &status_label, current_status);
                    let toast = Toast::new(&format!(
                        "Error: {}",
                        error.chars().take(100).collect::<String>()
                    ));
                    toast_overlay.add_toast(toast);
                }
                Err(_) => {
                    let current_status = check_tor_status(&cli_path);
                    update_ui_for_status(&button, &status_label, current_status);
                    let toast = Toast::new("An unexpected error occurred");
                    toast_overlay.add_toast(toast);
                }
            }
            let ip_info = get_ip_info().await;
            ip_label.set_text(&ip_info.ip);
            country_label.set_text(&ip_info.country);
            button.set_sensitive(true);
            power_spinner_inner.stop();
            power_spinner_inner.set_visible(false);
            power_button_inner.set_child(Some(power_icon_inner.as_ref()));
            power_icon_inner.set_visible(true);
        });
    });
}

pub fn update_ui_for_status(button: &Button, status_label: &gtk4::Label, is_active: bool) {
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
