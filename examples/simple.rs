#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate ezconf;

ezconf_file!(CONFIG = "tests/test.toml");

fn main() {
    let a = ezconf_str!(CONFIG: "string.a", "default");
    let b = ezconf_int!(CONFIG: "integer.b", 0);
    let c = ezconf_float!(CONFIG: "float.c", 0.0);
    let d = ezconf_bool!(CONFIG: "undefined.boolean", false);

    println!("String: {}", a);
    println!("Integer: {}", b);
    println!("Float: {}", c);
    println!("Boolean: {}", d);
}
