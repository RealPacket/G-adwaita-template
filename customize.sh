#!/usr/bin/sh
#     GAdwaita Template is a Rust GTK 4 + Adwaita template available on GitHub here:
#     <https://github.com/RealPacket/GAdwaita-template>
# Copyright (C) 2024  RealPacket / DataModel

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

read -p "Specify App ID: " -r app_id
read -p "Specify Main window title: " -r main_window_title
echo $0
echo "App ID: $app_id"
echo "Main Window Title: $main_window_title"
mv src/config.rs src/config.rs.backup
echo "renamed config.rs to config.rs.backup"
echo "
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
// Replace these using customize.sh (or just manually edit them)
pub const APP_ID: &str = \"$app_id\";
pub const MAIN_WINDOW_TITLE: &str = \"$main_window_title\";
" >> src/config.rs