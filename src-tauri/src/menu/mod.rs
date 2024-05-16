mod file;
mod help;
mod setting;
mod test;

use tauri::{
    api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind},
    Menu,
    WindowMenuEvent,
};

fn display_dialog(
    kind: MessageDialogKind,
    buttons: MessageDialogButtons,
    title: &str,
    msg: &str,
    handler: impl Fn(bool) + Send + 'static,
) {
    let dialog = tauri::api::dialog::MessageDialogBuilder::new(title, msg)
        .kind(kind)
        .buttons(buttons);
    dialog.show(handler);
}

#[macro_export]
macro_rules! create_menu {
    ($($module:ident),*) => {
        pub fn init_menu() -> Menu {
            Menu::with_items([
                $(  $module::new().into(), )*
            ])
        }

        pub fn event_handler(event: WindowMenuEvent) {
            $(
                if event.menu_item_id().starts_with(stringify!($module)) {
                    $module::event_handler(event);
                    return;
                }
            )+
                else {
                    println!("unknow menu event: {:?}", event.menu_item_id());
                }
        }
    };
}

create_menu!(file, setting, help, test);
