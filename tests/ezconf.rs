#[macro_use]
extern crate ezconf;
#[macro_use]
extern crate lazy_static;

ezconf_file!(CONFIG = "tests/test.toml");

#[test]
fn test_ezconf_str() {
    let name = ezconf_str!(CONFIG: "string.a", "Unnamed");
    let unavail = ezconf_str!(CONFIG: "string.unavail", "default");
    let version = ezconf_str!(CONFIG: "string.b", "Unversioned");

    assert!(name == "Foo");
    assert!(unavail == "default");
    assert!(version == "Bar")
}

#[test]
fn test_ezconf_int() {
    let val = ezconf_int!(CONFIG: "integer.a", 0);
    assert!(val == 1);
}

#[test]
fn test_ezconf_float() {
    let val = ezconf_float!(CONFIG: "float.a", 0.1);
    assert!((val - 2.0f64.sqrt()) < 0.1);
}

#[test]
fn test_ezconf_nonexistent() {
    ezconf_file!(MY_CFG = "foo/bar/non-existent.toml");

    assert_eq!(
        ezconf_int!(MY_CFG: "int.abc", 42),
        42,
    );
}


#[test]
fn test_multiple_basic() {
    ezconf_file!(CONFIG2 = "tests/test2.toml", "tests/test.toml");
    ezconf_file!(CONFIG3 = "tests/test.toml", "tests/test2.toml");

    assert_eq!(
        ezconf_int!(CONFIG: "integer.a", 0),
        1,
    );
    assert_eq!(
        ezconf_int!(CONFIG2: "integer.a", 0),
        3,
    );
    assert_eq!(
        ezconf_int!(CONFIG3: "integer.a", 0),
        1,
    );
}

#[test]
fn test_multiple_missing() {
    ezconf_file!(
        CFG = "tests/missing.toml",
        "tests/test2.toml",
        "tests/test.toml"
    );
    assert_eq!(
        ezconf_int!(CFG: "integer.a", 0),
        3,
    );
}

#[test]
fn test_multiple_all_missing() {
    ezconf_file!(
        CFG = "tests/missing1.toml",
        "tests/missing2.toml",
        "tests/missing3.toml"
    );
    assert_eq!(
        ezconf_int!(CFG: "integer.a", 0),
        0,
    );
}
