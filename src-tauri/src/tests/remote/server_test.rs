use tauri::async_runtime::block_on;

use super::{
    helper::{init_test_client, init_test_server},
    TEST_FILE_CONTENT,
    TEST_FILE_NAME,
    TEST_PASSWD,
};

#[test]
fn test_authorize_disconnect() {
    let mut server = init_test_server(TEST_FILE_CONTENT).unwrap();
    let mut client = init_test_client(server.get_port()).unwrap();
    let (filename, version, content) = block_on(client.send_authorize(TEST_PASSWD)).unwrap();
    assert_eq!(filename, TEST_FILE_NAME);
    assert_eq!(version, 0);
    assert_eq!(content, TEST_FILE_CONTENT);
    block_on(client.send_disconnect()).unwrap();
}

#[test]
fn test_set_cursor(){
    let mut server = init_test_server(TEST_FILE_CONTENT).unwrap();
    let mut client1= init_test_client(server.get_port()).unwrap();
    let mut client2= init_test_client(server.get_port()).unwrap();
    let mut client3= init_test_client(server.get_port()).unwrap();
}