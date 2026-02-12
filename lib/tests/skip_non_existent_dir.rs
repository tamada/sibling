use std::path::Path;

mod common;

#[test]
pub fn test_skip_non_existent_dir() {
    common::init();
    let _ = std::env::set_current_dir(Path::new("../testdata/basic"));
    let dirs =
        sibling::Dirs::new_from_file("../skip.txt").expect("Failed to create Dirs from skip.txt");
    assert_eq!(dirs.len(), 2);

    let dir = dirs
        .next(sibling::NexterFactory::create(sibling::NexterType::Next).as_ref())
        .expect("Failed to get next directory");
    assert_eq!(dir.path().to_string_lossy(), "worried");
}

#[test]
pub fn test_skip_non_existent_dir2() {
    common::init();
    let _ = std::env::set_current_dir(Path::new("../testdata/basic"));
    let dirs = sibling::Dirs::new_from_file_with("../skip.txt", true)
        .expect("Failed to create Dirs from skip.txt");
    assert_eq!(dirs.len(), 4);

    let dir = dirs
        .next(sibling::NexterFactory::create(sibling::NexterType::Previous).as_ref())
        .expect("Failed to get next directory");
    assert_eq!(dir.path().to_string_lossy(), "unknown_dir");
}
