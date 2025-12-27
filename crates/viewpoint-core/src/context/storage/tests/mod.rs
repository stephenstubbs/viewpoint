use super::*;

#[test]
fn test_storage_state_options() {
    let options = StorageStateOptions::new()
        .indexed_db(true)
        .indexed_db_max_entries(500);

    assert!(options.indexed_db);
    assert_eq!(options.indexed_db_max_entries, 500);
}
