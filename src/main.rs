/*
	GAdwaita Template is a Rust GTK 4 + Adwaita template available on GitHub here:
	<https://github.com/RealPacket/GAdwaita-template>
Copyright (C) 2024  RealPacket / DataModel

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

mod config;

use adw::{prelude::*, ActionRow, Application, ApplicationWindow, HeaderBar, WindowTitle};
use config::{APP_ID, MAIN_WINDOW_TITLE};
use gtk::{Box, ListBox, Orientation};

fn main() {
	let application = Application::builder().application_id(APP_ID).build();

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
		content.append(&list);
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
