mod config;

use config::{APP_ID, MAIN_WINDOW_TITLE};
use gtk4::{Application, Box, ListBox, Orientation};
use libadwaita::{init, prelude::*, ActionRow, ApplicationWindow, HeaderBar, WindowTitle};

fn main() {
    let application = Application::builder().application_id(APP_ID).build();

    application.connect_startup(|_| {
        // initialize Adwaita
        init().unwrap();
    });

    application.connect_activate(|app| {
        // ActionRows are only available in Adwaita
        let row = ActionRow::builder()
            .activatable(true)
            .selectable(false)
            .title("Click to print \"Activated!\"")
            .build();
        row.connect_activated(|_| {
            eprintln!("Activated!");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            // the content class makes the list look nicer
            .css_classes(vec![String::from("content")])
            .build();
        list.append(&row);

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaita's ApplicationWindow does not include a HeaderBar
        content.append(
            &HeaderBar::builder()
                .title_widget(&WindowTitle::new(MAIN_WINDOW_TITLE, ""))
                .build(),
        );
        // TODO(finalize): append your content

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.show();
    });

    application.run();
}
