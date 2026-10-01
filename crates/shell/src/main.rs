use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Box, Button, Label, Orientation};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

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

    LayerShell::init(&window);
    window.set_layer(Layer::Top);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_exclusive_zone(BAR_HEIGHT);
    window.set_keyboard_interactivity(false);

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

    let launcher = Button::builder().label("Launch").build();
    launcher.add_css_class("myos-launcher");
    launcher.connect_clicked(|_| {
        let _ = std::process::Command::new("sh")
            .args(["-c", "${TERMINAL:-foot} &"])
            .spawn();
    });
    bar.append(&launcher);

    window.set_child(Some(&bar));
    install_css();
    start_clock(clock);
    window.present();
}

fn install_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        ".myos-bar { background: rgba(12, 18, 24, 0.94); color: #e7f0ed; border-bottom: 1px solid #29423f; }\n.myos-brand { color: #7de2c3; font-weight: 800; letter-spacing: 2px; }\n.myos-clock { color: #b9d8d0; font-variant-numeric: tabular-nums; }\n.myos-launcher { background: #d6f36b; color: #132018; border-radius: 6px; padding: 5px 12px; font-weight: 700; }",
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
