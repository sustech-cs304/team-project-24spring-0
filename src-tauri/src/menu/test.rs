use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

use crate::{
    modules::riscv::middleware::frontend_api::{
        authorize_share_client,
        start_share_server,
        stop_share_server,
    },
    types::middleware_types::{CurTabName, TabMap},
};

pub fn new() -> Submenu {
    Submenu::new(
        "Test",
        Menu::with_items([
            CustomMenuItem::new("test_foo", "StartServer").into(),
            CustomMenuItem::new("test_oo", "StartClient").into(),
            CustomMenuItem::new("test_bar", "Stop").into(),
        ]),
    )
}

pub fn event_handler(event: WindowMenuEvent) {
    match event.menu_item_id().strip_prefix("test_").unwrap() {
        "foo" => {
            let window = event.window();
            start_share_server(
                window.state::<CurTabName>(),
                window.state::<TabMap>(),
                11451,
                "foo",
            );
        }
        "bar" => {
            let window = event.window();
            if !stop_share_server(window.state::<TabMap>()) {
                println!("Share server is not running.");
            }
        }
        "oo" => {
            let window = event.window();
            let res = authorize_share_client(
                window.to_owned(),
                window.state::<CurTabName>(),
                window.state::<TabMap>(),
                "127.0.0.1".to_string(),
                11451,
                "foo".to_string(),
            );
            println!("authorize_share_client: {:?}", res);
        }
        _ => {}
    }
}
