use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Box, Button, Label, Orientation, Popover};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

const APP_ID: &str = "com.myos.Shell";
const BAR_HEIGHT: i32 = 52;

fn main() -> glib::ExitCode {
    let application = Application::builder().application_id(APP_ID).build();
    application.connect_activate(build_shell);
    application.run()
}

fn build_shell(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("MyOS shell")
        .default_height(BAR_HEIGHT)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_exclusive_zone(BAR_HEIGHT);
    window.set_keyboard_mode(KeyboardMode::None);

    let bar = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .margin_start(18)
        .margin_end(18)
        .margin_top(8)
        .margin_bottom(8)
        .build();
    bar.add_css_class("myos-bar");

    let brand = Label::builder().label("MYOS").build();
    brand.add_css_class("myos-brand");
    bar.append(&brand);

    let spacer = Box::builder().hexpand(true).build();
    bar.append(&spacer);

    let clock = Label::builder().label("--:--").build();
    clock.add_css_class("myos-clock");
    bar.append(&clock);

    let launcher = Button::builder().label("Open").build();
    launcher.add_css_class("myos-launcher");
    let launcher_menu = Popover::new();
    launcher_menu.set_parent(&launcher);
    launcher_menu.set_child(Some(&launcher_menu_content(&launcher_menu)));
    let launcher_menu_for_button = launcher_menu.clone();
    launcher.connect_clicked(move |_| launcher_menu_for_button.popup());
    bar.append(&launcher);

    let widgets = Button::builder().label("Widgets").build();
    widgets.add_css_class("myos-secondary");
    let widgets_menu = Popover::new();
    widgets_menu.set_parent(&widgets);
    widgets_menu.set_child(Some(&widget_content()));
    let widgets_menu_for_button = widgets_menu.clone();
    widgets.connect_clicked(move |_| widgets_menu_for_button.popup());
    bar.append(&widgets);

    window.set_child(Some(&bar));
    install_css();
    start_clock(clock);
    window.present();
}

fn launcher_menu_content(menu: &Popover) -> Box {
    let content = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    content.add_css_class("myos-popover");

    for (label, command) in [("Terminal", "${TERMINAL:-foot}"), ("Files", "${FILE_MANAGER:-nautilus}")] {
        let button = Button::with_label(label);
        button.add_css_class("myos-menu-button");
        let command = command.to_string();
        let menu = menu.clone();
        button.connect_clicked(move |_| {
            spawn_command(&command);
            menu.popdown();
        });
        content.append(&button);
    }

    content
}

fn widget_content() -> Box {
    let content = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(14)
        .margin_end(14)
        .build();
    content.add_css_class("myos-popover");

    let heading = Label::builder().label("SYSTEM PULSE").halign(gtk4::Align::Start).build();
    heading.add_css_class("myos-widget-heading");
    content.append(&heading);
    for text in ["Network   ready", "Audio      ready", "Battery    checking"] {
        let label = Label::builder().label(text).halign(gtk4::Align::Start).build();
        label.add_css_class("myos-widget-row");
        content.append(&label);
    }

    content
}

fn spawn_command(command: &str) {
    let _ = std::process::Command::new("sh")
        .args(["-c", &format!("{command} &")])
        .spawn();
}

fn install_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(
        ".myos-bar { background: rgba(12, 18, 24, 0.94); color: #e7f0ed; border-bottom: 1px solid #29423f; }\n.myos-brand { color: #7de2c3; font-weight: 800; letter-spacing: 2px; }\n.myos-clock { color: #b9d8d0; font-variant-numeric: tabular-nums; }\n.myos-launcher { background: #d6f36b; color: #132018; border-radius: 6px; padding: 5px 12px; font-weight: 700; }\n.myos-secondary { background: #203532; color: #d5e9e3; border-radius: 6px; padding: 5px 12px; }\n.myos-popover { background: #13211f; border: 1px solid #41625b; border-radius: 8px; }\n.myos-menu-button { color: #e7f0ed; min-width: 150px; }\n.myos-widget-heading { color: #7de2c3; font-size: 0.8em; font-weight: 800; letter-spacing: 1px; }\n.myos-widget-row { color: #c9d8d3; font-variant-numeric: tabular-nums; }",
    );
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Wayland display is required"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn start_clock(clock: Label) {
    glib::timeout_add_seconds_local(1, move || {
        let now = glib::DateTime::now_local().expect("local time should be available");
        clock.set_label(&now.format("%H:%M").expect("clock format should be valid"));
        glib::ControlFlow::Continue
    });
}
