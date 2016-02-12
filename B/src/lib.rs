extern crate A;

use A::Foo;

struct Bar;

impl Foo for Bar {
    fn foo(&self) -> u32 {
        return 5;
    }
}
