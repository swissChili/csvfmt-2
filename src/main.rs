extern crate iui;
extern crate nfd;
extern crate csv;
use iui::prelude::*;
use iui::controls::{Label, Button, VerticalBox};
use nfd::Response;
use std::fs::File;
use std::io::prelude::*;

fn fmt_csv_file(path: String) {
    let mut rdr = csv::Reader::from_path(path.clone()).unwrap();

    let mut out = String::from("Tracking Number,Carrier,Recipient,Email\n");

    for result in rdr.records() {
        let record: csv::StringRecord = result.expect("Could not read record from CSV");

        let line = format!("{},{},{},{}\n",
            record.get(3).unwrap(), record.get(24).unwrap(), record.get(0).unwrap(), record.get(2).unwrap()
        );
        out.push_str(&line);
    }

    let mut f = File::create(format!("{}_out.csv", path)).expect("Could not create file");
    f.write_all(out.as_bytes()).expect("Could not write to file");
}

fn fmt_win() {
    let result = nfd::dialog_multiple().filter("csv").open().expect("Couldn't open file dialog.");

    match result {
        Response::Okay(file_path) => fmt_csv_file(file_path),
        Response::OkayMultiple(file_paths) => {
            for file in file_paths {
                fmt_csv_file(file);
            }
        },
        Response::Cancel => println!("User Canceled"),
    };
}

fn main() {
    // Initialize the UI library
    let ui = UI::init().expect("Couldn't initialize UI library");
    // Create a window into which controls can be placed
    let mut win = Window::new(&ui, "CSV Formatter", 200, 200, WindowType::NoMenubar);
    
    // Create a vertical layout to hold the controls
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    // Create two buttons to place in the window
    let mut button = Button::new(&ui, "Open CSV");
    button.on_clicked(&ui, {
        |_| {
            fmt_win();
        }
    });

    let mut quit_button = Button::new(&ui, "Quit");
    quit_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
        }
    });

    // Create a new label. Note that labels don't auto-wrap!
    let mut label_text = String::new();
    label_text.push_str("CSV Formatter v0.1.0\n");
    label_text.push_str("Copyright © 2019 swisschili.sh");
    let label = Label::new(&ui, &label_text);

    vbox.append(&ui, label, LayoutStrategy::Stretchy);
    vbox.append(&ui, button, LayoutStrategy::Compact);
    vbox.append(&ui, quit_button, LayoutStrategy::Compact);

    // Actually put the button in the window
    win.set_child(&ui, vbox);
    // Show the window
    win.show(&ui);
    // Run the application
    ui.main();
}
