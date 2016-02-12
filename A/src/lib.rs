pub trait Foo {
    // Change u32 to f32. The dependent crate *should* fail to compile afterwards.
    fn foo(&self) -> u32;
}
