use localtrust_core::{Action, Broker, gguf_version};
fn create(path: &str) -> Action { Action::Create { path: path.into(), content: "Synthetic meeting notes".into() } }
#[test] fn approval_is_required_and_single_use() {
    let dir = tempfile::tempdir().unwrap(); let mut b = Broker::new(dir.path()).unwrap();
    let p = b.propose(create("notes.txt")).unwrap(); assert!(!dir.path().join("notes.txt").exists());
    b.decide(p.id, true).unwrap(); assert_eq!(std::fs::read_to_string(dir.path().join("notes.txt")).unwrap(), "Synthetic meeting notes");
    assert!(b.decide(p.id, true).is_err());
    let p = b.propose(Action::Read { path: "notes.txt".into() }).unwrap();
    assert_eq!(b.decide(p.id, true).unwrap(), "Synthetic meeting notes");
}
#[test] fn denial_and_unknown_id_never_write() {
    let dir = tempfile::tempdir().unwrap(); let mut b = Broker::new(dir.path()).unwrap();
    let p = b.propose(create("notes.txt")).unwrap(); b.decide(p.id, false).unwrap();
    assert!(!dir.path().join("notes.txt").exists()); assert!(b.decide(900, true).is_err());
    assert_eq!(b.events()[0].outcome, "denied");
}
#[test] fn rejects_unsafe_paths_and_overwrites() {
    let dir = tempfile::tempdir().unwrap(); let mut b = Broker::new(dir.path()).unwrap();
    for path in ["../x.txt", "a/b.txt", "C:\\a.txt", "x.txt:stream", "CON.txt", "a.exe", ""] { assert!(b.propose(create(path)).is_err(), "{path}"); }
    let p = b.propose(create("x.txt")).unwrap(); std::fs::write(dir.path().join("x.txt"), "existing").unwrap();
    assert!(b.decide(p.id, true).is_err()); assert_eq!(std::fs::read_to_string(dir.path().join("x.txt")).unwrap(), "existing");
}
#[test] fn bounds_content_and_reads() {
    let dir = tempfile::tempdir().unwrap(); let mut b = Broker::new(dir.path()).unwrap();
    assert!(b.propose(Action::Create { path: "x.txt".into(), content: "x".repeat(32769) }).is_err());
    std::fs::write(dir.path().join("big.txt"), "x".repeat(32769)).unwrap();
    let p = b.propose(Action::Read { path: "big.txt".into() }).unwrap(); assert!(b.decide(p.id, true).is_err());
}
#[test] fn strict_model_schema() {
    for json in [r#"{"action":"shell","command":"echo bad"}"#, r#"{"action":"read","path":"a.txt","extra":true}"#] { assert!(serde_json::from_str::<Action>(json).is_err()); }
}
#[test] fn cpp_prefix_validation() {
    assert!(gguf_version(b"GGUF").is_err()); let mut data = [0u8; 24]; data[..4].copy_from_slice(b"GGUF");
    data[4] = 3; assert_eq!(gguf_version(&data).unwrap(), 3); data[4] = 9; assert!(gguf_version(&data).is_err());
}
#[cfg(unix)]
#[test] fn rejects_symbolic_links() {
    let dir = tempfile::tempdir().unwrap(); let outside = tempfile::NamedTempFile::new().unwrap();
    std::os::unix::fs::symlink(outside.path(), dir.path().join("link.txt")).unwrap();
    let mut b = Broker::new(dir.path()).unwrap(); assert!(b.propose(Action::Read { path: "link.txt".into() }).is_err());
}
