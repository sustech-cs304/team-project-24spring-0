mod menu_file;
use tauri::{Menu, WindowMenuEvent};

#[macro_export]
macro_rules! create_menu_and_handler {
    ($($module:ident),+) => {
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

create_menu_and_handler!(menu_file);
