mod local_test {
    use std::{str::FromStr, thread, time::Duration};

    use tauri::async_runtime::block_on;

    use super::super::{
        helper::{init_test_client, init_test_server},
        TEST_FILE_CONTENT,
        TEST_FILE_NAME,
        TEST_PASSWD,
    };
    use crate::{
        remote::{server::editor_rpc::OperationType, Modification, OpRange},
        tests::remote::{helper::insert_str_at_utf8_char_index, TAB_MAP},
        types::rpc_types::CursorPosition,
    };

    #[test]
    fn test_authorize_disconnect() {
        let mut server = init_test_server(TEST_FILE_CONTENT).unwrap();
        thread::sleep(Duration::from_secs(2));
        let mut client = init_test_client(server.get_port()).unwrap();

        let (filename, version, content) = block_on(client.send_authorize(TEST_PASSWD)).unwrap();
        assert_eq!(filename, TEST_FILE_NAME);
        assert_eq!(version, 0);
        assert_eq!(content, TEST_FILE_CONTENT);

        let _ = block_on(client.send_disconnect()).unwrap();
        client.stop().unwrap();
        thread::sleep(Duration::from_secs(2));
        server.stop_server();
    }

    #[test]
    fn test_update_and_get_content() {
        let mut server = init_test_server(TEST_FILE_CONTENT).unwrap();
        thread::sleep(Duration::from_secs(2));
        let mut client1 = init_test_client(server.get_port()).unwrap();

        let (filename, version, content) = block_on(client1.send_authorize(TEST_PASSWD)).unwrap();
        assert_eq!(filename, TEST_FILE_NAME);
        assert_eq!(version, 0);
        assert_eq!(content, TEST_FILE_CONTENT);

        let res = block_on(client1.send_update_content(
            0,
            &Modification {
                op: OperationType::Insert,
                version: 0,
                op_range: OpRange {
                    start: CursorPosition { row: 0, col: 5 },
                    end: CursorPosition { row: 0, col: 0 },
                },
                modified_content: "Test".to_string(),
            },
        ));
        assert_eq!(res.as_ref().unwrap().success, true);
        {
            let tab_map_lock = TAB_MAP.lock().unwrap();
            let tab_maps = tab_map_lock.as_ref().unwrap();
            let mut tabs = tab_maps.tabs.lock().unwrap();
            let mut tab = tabs.get_mut(TEST_FILE_NAME).unwrap();
            let mut expected = String::from_str(TEST_FILE_CONTENT).unwrap();
            insert_str_at_utf8_char_index(&mut expected, "Test", 5);
            let res = tab.text.get_raw().to_string();
            assert_eq!(res, expected);
        }
        let res = block_on(client1.send_update_content(
            0,
            &Modification {
                op: OperationType::Insert,
                version: 0,
                op_range: OpRange {
                    start: CursorPosition { row: 0, col: 5 },
                    end: CursorPosition { row: 0, col: 0 },
                },
                modified_content: "Bar".to_string(),
            },
        ));
        assert_eq!(res.as_ref().unwrap().success, false);

        let res = block_on(client1.send_get_content(0));
        assert_eq!(res.as_ref().unwrap().history.len(), 1);
        assert_eq!(res.as_ref().unwrap().history[0].version, 0);
        assert_eq!(res.as_ref().unwrap().history[0].modified_content, "Test");

        let _ = block_on(client1.send_disconnect());
        client1.stop().unwrap();
        thread::sleep(Duration::from_secs(2));
        server.stop_server();
    }
}
