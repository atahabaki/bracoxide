use bracoxide::explode;

#[test]
fn numeric_range_1_to_3() {
    let pos = explode("{1..3}");
    assert!(pos.is_ok());
    assert_eq!(
        pos.unwrap(),
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    )
}

#[test]
fn numeric_range_3_to_1() {
    let pos = explode("{3..1}");
    assert!(pos.is_ok());
    assert_eq!(
        pos.unwrap(),
        vec!["3".to_string(), "2".to_string(), "1".to_string()]
    )
}

#[test]
fn numeric_range_0_to_3() {
    let pos = explode("{..3}");
    assert!(pos.is_ok());
    assert_eq!(
        pos.unwrap(),
        vec![
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string()
        ]
    )
}

#[test]
fn numeric_range_3_to_0() {
    let pos = explode("{3..}");
    assert!(pos.is_ok());
    assert_eq!(
        pos.unwrap(),
        vec![
            "3".to_string(),
            "2".to_string(),
            "1".to_string(),
            "0".to_string()
        ]
    )
}
