use bracoxide::explode;

#[test]
fn simple_a_b() {
    let pos = explode("{A,B}");
    assert!(pos.is_ok());
    assert_eq!(pos.unwrap(), vec!["A".to_string(), "B".to_string()]);
}

#[test]
fn simple_a_b_c_d_e_f_g() {
    let pos = explode("{A,B,C,D,E,F,G}");
    assert_eq!(
        pos.unwrap(),
        vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
            "G".to_string()
        ]
    );
}
