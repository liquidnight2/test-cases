#[macro_use]
extern crate rust_i18n;

i18n!("locales");

fn main() {
    println!("translated={}", t!("hello"));
    assert_eq!(t!("hello"), "Hello_World-English!");
}
