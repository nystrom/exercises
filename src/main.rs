#![allow(unused_variables)]

// DO NOT CHANGE THIS DEFINITION
#[derive(Debug, PartialEq)]
struct Foo(i32);

// TODO: modify the argument type to make the compile errors in main go away.
fn use_foo(foo: Foo) {
    println!("uses_foo: {:?}", foo);
}

// TODO: modify the argument type to make the compile errors in main go away.
// TODO: implement this function, so that the assert in main does not fail.
fn change_foo(foo: Foo) {
}

// TODO You might need to change this function too.
// Change as LITTLE as possible.
fn main() {
    let x = Foo(1);
    let y = Foo(2);

    use_foo(x);
    change_foo(y);
    use_foo(x);

    assert_eq!(x, y);
}
