use tempdir::TempDir;

use soter::dir::DirStorage;

#[test]
fn test() {
    let dir = TempDir::new("soter_test").unwrap();
    let dir_str = dir.path().to_str().unwrap();

    let mut dir_storage: DirStorage<u32> = DirStorage::default();
    dir_storage.insert("1", 1);
    dir_storage.insert("2", 2);
    dir_storage.insert("3", 3);

    dir_storage.store(dir_str).unwrap();

    let new_dir_storage: DirStorage<u32> = DirStorage::restore(dir_str).unwrap();

    assert_eq!(*new_dir_storage.get("1").unwrap(), 1);
    assert_eq!(*new_dir_storage.get("2").unwrap(), 2);
    assert_eq!(*new_dir_storage.get("3").unwrap(), 3);
}
