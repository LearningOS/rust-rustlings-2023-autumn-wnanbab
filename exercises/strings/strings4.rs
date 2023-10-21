// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    let a =string_slice("blue");
    let b =string("red".to_string());
    let c =string(String::from("hi"));
    let d =string("rust is fun!".to_owned());
    let e =string_slice("nice weather".into());
    let f =string(format!("Interpolation {}", "Station"));
    let g =string_slice(&String::from("abc")[0..1]);
    let h =string_slice("  hello there ".trim());
    let i =string("Happy Monday!".to_string().replace("Mon", "Tues"));
    let g =string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
