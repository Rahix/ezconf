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
