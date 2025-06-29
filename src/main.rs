use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Entry, Grid, Label, glib};
const APP_ID: &str = "org.gtk_rs.HelloWorld3";

use std::cell::RefCell;
use std::rc::Rc;

pub struct Wordle {
    answer: String,
    round: u8,
}

impl Wordle {
    //TO DO Change this function to also if not correct, what spaces are correct"
    pub fn check_string(&self, guess: &String) -> bool {
        self.answer == guess.clone()
    }
    pub fn correct_slots(&self, guess: &String) -> Vec<u8> {
        let answer_char = self.answer.as_str().chars();
        let guess_clone = guess.clone();
        let guess_char = guess_clone.as_str().chars();
        let mut count = 0;
        let mut correct: Vec<u8> = Vec::new();
        for (c_answer, c_guess) in answer_char.zip(guess_char) {
            if c_answer == c_guess {
                correct.push(count);
            }
            count += 1;
        }
        correct
    }

    fn update_round(&mut self) {
        self.round += 1;
    }

    pub fn get_round(&self) -> u8 {
        self.round
    }
}

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

    let mut entry_vec = Vec::new();

    grid.attach(&label, 0, 0, 10, 1);
    for i in 1..6 {
        for j in 1..6 {
            let entry = Entry::new();
            entry.set_max_length(1);
            grid.attach(&entry, j, i + 1, 1, 1);
            entry_vec.push(entry);
        }
    }
    let button = Button::builder()
        .label("Submit Guess!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let entries_clone = entry_vec.clone();
    //TO-DO load this with a random value from a 5 letter word list
    let game = Rc::new(RefCell::new(Wordle {
        answer: "hello".to_owned(),
        round: 0,
    }));

    let game_clone = Rc::clone(&game);
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        let mut screen_string = String::new();

        for (i, entry) in entries_clone.iter().enumerate() {
            let text = entry.text();
            if !text.is_empty() {
                screen_string.push_str(text.as_str());
                entry.set_editable(false);
            }
            println!("Entry {}: {}", i, text);
        }
        let mut game = game_clone.borrow_mut();
        let check = game.check_string(&screen_string);
        let correct = game.correct_slots(&screen_string);
        for val in correct {
            println!("{val}")
        }
        game.update_round();
        let round = game.get_round();
        println!("{round}");
    });

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
