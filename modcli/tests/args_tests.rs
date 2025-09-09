use modcli::args;
use modcli::error::ModCliError;

#[test]
fn flag_detects_presence_and_truthy_equal() {
    let argv = vec![
        "--alpha".to_string(),
        "--beta=true".to_string(),
        "--gamma=YES".to_string(),
        "--delta=0".to_string(),
    ];
    assert!(args::flag(&argv, "--alpha"));
    assert!(args::flag(&argv, "--beta"));
    assert!(args::flag(&argv, "--gamma"));
    assert!(!args::flag(&argv, "--delta"));
    assert!(!args::flag(&argv, "--missing"));
}

#[test]
fn get_string_reads_split_and_equals() {
    let argv = vec![
        "--name".to_string(),
        "james".to_string(),
        "--host=localhost".to_string(),
    ];
    assert_eq!(args::get_string(&argv, "--name").as_deref(), Some("james"));
    assert_eq!(args::get_string(&argv, "--host").as_deref(), Some("localhost"));
    assert_eq!(args::get_string(&argv, "--missing"), None);
}

#[test]
fn get_int_parses_and_errors() {
    let ok = vec!["--port=8080".to_string()];
    let bad = vec!["--port".to_string(), "abc".to_string()];
    let miss: Vec<String> = vec![];

    let p: u16 = args::get_int(&ok, "--port").unwrap();
    assert_eq!(p, 8080);

    let e = args::get_int::<u16>(&bad, "--port").unwrap_err();
    match e {
        ModCliError::InvalidUsage(msg) => assert!(msg.contains("expected numeric")),
        other => panic!("unexpected error: {other:?}"),
    }

    let e2 = args::get_int::<u16>(&miss, "--port").unwrap_err();
    match e2 {
        ModCliError::InvalidUsage(msg) => assert!(msg.contains("missing required argument")),
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn get_bool_supports_key_and_equals() {
    let argv = vec![
        "--verbose".to_string(),
        "--debug=false".to_string(),
        "--trace=Yes".to_string(),
    ];
    assert_eq!(args::get_bool(&argv, "--verbose").unwrap(), true);
    assert_eq!(args::get_bool(&argv, "--debug").unwrap(), false);
    assert_eq!(args::get_bool(&argv, "--trace").unwrap(), true);
}
