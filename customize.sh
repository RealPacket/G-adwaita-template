#!/usr/bin/sh
read -p "Specify App ID: " -r app_id
read -p "Specify Main window title: " -r main_window_title
echo $0
echo "App ID: $app_id"
echo "Main Window Title: $main_window_title"
mv src/config.rs src/config.rs.backup
echo "renamed config.rs to config.rs.backup"
echo "// Replace these using customize.sh (or just manually edit them)
pub const APP_ID: &str = \"$app_id\";
pub const MAIN_WINDOW_TITLE: &str = \"$main_window_title\";
" >> src/config.rs