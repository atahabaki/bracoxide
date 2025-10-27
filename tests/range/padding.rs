use bracoxide::explode;

#[test]
fn range_padding_zero_90_to_100() {
    let pos = explode("{90..100;0=}");
    assert!(pos.is_ok());
    assert_eq!(
        pos.unwrap(),
        vec![
            "090".to_string(),
            "091".to_string(),
            "092".to_string(),
            "093".to_string(),
            "094".to_string(),
            "095".to_string(),
            "096".to_string(),
            "097".to_string(),
            "098".to_string(),
            "099".to_string(),
            "100".to_string(),
        ]
    );
}

#[test]
fn range_padding_zero_9_to_10() {
    let pos = explode("{7..10;100_00=}");
    assert!(pos.is_ok());
    assert_eq!(
        pos.unwrap(),
        vec![
            "100_007".to_string(),
            "100_008".to_string(),
            "100_009".to_string(),
            "100_010".to_string(),
        ]
    );
}

#[test]
fn range_padding_zero_10_to_7() {
    let pos = explode("{10..7;=00_000}");
    assert!(pos.is_ok());
    assert_eq!(
        pos.unwrap(),
        vec![
            "700_000".to_string(),
            "800_000".to_string(),
            "900_000".to_string(),
            "100_000".to_string(),
        ]
    );
}
