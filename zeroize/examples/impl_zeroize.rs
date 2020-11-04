use zeroize::{DefaultIsZeroes, Zeroize};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Foo {
    f1: u8,
    f2: u8,
}

impl Default for Foo {
    fn default() -> Self {
        Foo { f1: 42, f2: 42 }
    }
}

impl DefaultIsZeroes for Foo {}

impl Zeroize for Foo {
    fn zeroize(&mut self) {
        self.default_zeroize();
    }
}

fn main() {
    let mut foo = Foo::default();
    foo.f1 = 0;
    foo.zeroize();
    assert_eq!(foo, Foo::default());
}
