//! Integration tests for `zeroize_derive` proc macros

use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
struct ZeroizableTupleStruct([u8; 3]);

#[test]
fn derive_tuple_struct_test() {
    let mut value = ZeroizableTupleStruct([1, 2, 3]);
    value.zeroize();
    assert_eq!(&value.0, &[0, 0, 0])
}

#[derive(Zeroize, ZeroizeOnDrop)]
struct ZeroizableStruct {
    string: String,
    vec: Vec<u8>,
    bytearray: [u8; 3],
    number: usize,
    boolean: bool,
}

#[test]
fn derive_struct_test() {
    let mut value = ZeroizableStruct {
        string: String::from("Hello, world!"),
        vec: vec![1, 2, 3],
        bytearray: [4, 5, 6],
        number: 42,
        boolean: true,
    };

    value.zeroize();

    assert!(value.string.is_empty());
    assert!(value.vec.is_empty());
    assert_eq!(&value.bytearray, &[0, 0, 0]);
    assert_eq!(value.number, 0);
    assert!(!value.boolean);
}
