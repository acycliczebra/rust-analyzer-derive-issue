#![allow(dead_code, clippy::no_effect)]
pub use my_derive::WithIgnoredFields;

#[derive(WithIgnoredFields)]
struct MyStruct {
    field_a: &'static str,
    #[ignore_field]
    field_b: &'static str,
}

fn main() {
    MyStructWithIgnoredFields {
        field_a: "Hello World",
    };
}
