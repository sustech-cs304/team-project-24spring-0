mod menu_file;

use tauri::api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind};
use tauri::{Menu, WindowMenuEvent};

fn display_alert_dialog(kind: MessageDialogKind, title: &str, msg: &str, handler: fn(bool)) {
    let dialog = MessageDialogBuilder::new(title, msg)
        .kind(kind)
        .buttons(MessageDialogButtons::Ok);
    dialog.show(handler);
}

fn display_confirm_dialog(kind: MessageDialogKind, title: &str, msg: &str, handler: fn(bool)) {
    let dialog = tauri::api::dialog::MessageDialogBuilder::new(title, msg)
        .buttons(MessageDialogButtons::OkCancel);
    dialog.show(handler);
}

#[macro_export]
macro_rules! create_menu {
    ($($module:ident),*) => {
        pub fn init_menu() -> Menu {
            Menu::with_items([
                $(  $module::new().into(), )+
            ])
        }

        pub fn event_handler(event: WindowMenuEvent) {
            $(
                if event.menu_item_id().starts_with(stringify!($module).strip_prefix("menu_").unwrap()) {
                    $module::event_handler(event);
                }
            )+
                else {
                    println!("unknow menu event: {:?}", event.menu_item_id());
                }
        }
    };
}

create_menu!(menu_file);
