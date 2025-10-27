use bracoxide::explode;

#[test]
fn char_range_a_to_c() {
    let pos = explode("{a..c}");
    assert!(pos.is_ok());
    assert_eq!(
        pos.unwrap(),
        vec!["a".to_string(), "b".to_string(), "c".to_string(),]
    )
}

#[test]
fn char_range_c_to_a() {
    let pos = explode("{c..a}");
    assert!(pos.is_ok());
    assert_eq!(
        pos.unwrap(),
        vec!["c".to_string(), "b".to_string(), "a".to_string(),]
    )
}

#[test]
fn char_range_c_to() {
    let pos = explode("{c..}");
    assert!(pos.is_ok());
    assert_eq!(
        pos.unwrap(),
        vec!["c".to_string(), "b".to_string(), "a".to_string(),]
    )
}

#[test]
fn char_range_to_c() {
    let pos = explode("{..c}");
    assert!(pos.is_ok());
    assert_eq!(
        pos.unwrap(),
        vec!["a".to_string(), "b".to_string(), "c".to_string(),]
    )
}
