use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Entry, Grid, Label, glib};
const APP_ID: &str = "org.gtk_rs.HelloWorld3";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Submit Guess!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    let label = Label::new(Some("Wordle-RS"));

    label.set_halign(gtk4::Align::Start);
    label.set_margin_bottom(10);
    label.set_markup("<span size='xx-large' weight='bold'>Wordle-RS</span>"); // bold & big text
    let grid = Grid::builder()
        .column_spacing(10)
        .row_spacing(10)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    grid.attach(&label, 0, 0, 10, 1);
    for i in 1..6 {
        for j in 1..6 {
            let entry = Entry::new();
            grid.attach(&entry, j, i + 1, 1, 1);
        }
    }
    grid.attach(&button, 3, 7, 1, 1);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&grid)
        .build();

    // Present window
    window.present();
}
