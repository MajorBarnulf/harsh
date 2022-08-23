use super::*;
#[test]
fn test_list() {
    let db = sled::open("/tmp/test-db").unwrap();
    db.insert("/some/path/123", b"hello1").unwrap();
    db.insert("/some/path/1234", b"hello2").unwrap();
    db.insert("/some/path/12345", b"hello3").unwrap();
    let results = list(&db, "/some/path/".to_string());
    assert_eq!(
        results,
        vec![
            Id::from_string("123").unwrap(),
            Id::from_string("1234").unwrap(),
            Id::from_string("12345").unwrap()
        ]
    );
}

#[tokio::test]
async fn test_channels() {
    use telecomande::{Executor, SimpleExecutor};
    // cleaning;
    std::fs::remove_dir_all("/tmp/db-test").ok();

    // instantiation
    let store = SimpleExecutor::new(StorageProc::new("/tmp/db-test")).spawn();
    let remote = store.remote();

    // insertion
    let (cmd, rec) = StorageCmd::new_channel_create("a-channel");
    remote.send(cmd).unwrap();
    let id = rec.await.unwrap();

    // query all
    let (cmd, rec) = StorageCmd::new_channel_list();
    remote.send(cmd).unwrap();
    let result = rec.await.unwrap();
    assert_eq!(result.len(), 1);
    let first = result[0];
    assert_eq!(first, id);

    // query property
    let (cmd, rec) = StorageCmd::new_channel_get_name(id);
    remote.send(cmd).unwrap();
    let result = rec.await.unwrap();
    assert_eq!(result.unwrap(), "a-channel".to_string());

    // insertion
    let (cmd, rec) = StorageCmd::new_channel_create("b-channel");
    remote.send(cmd).unwrap();
    let id2 = rec.await.unwrap();

    // query all
    let (cmd, rec) = StorageCmd::new_channel_list();
    remote.send(cmd).unwrap();
    let result = rec.await.unwrap();
    assert_eq!(result.len(), 2);

    // query property
    let (cmd, rec) = StorageCmd::new_channel_get_name(id2);
    remote.send(cmd).unwrap();
    let result = rec.await.unwrap();
    assert_eq!(result.unwrap(), "b-channel".to_string());
}
