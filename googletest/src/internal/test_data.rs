//! This is for testing only to provide fully-qualified struct paths to macros.

#[doc(hidden)]
#[derive(Debug)]
pub struct TestStruct {
    #[doc(hidden)]
    pub value: u32,
}

impl TestStruct {
    #[doc(hidden)]
    pub fn get_value(&self) -> u32 {
        self.value
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct GenericTestStruct<T> {
    #[doc(hidden)]
    pub value: T,
}

impl<T> GenericTestStruct<T> {
    #[doc(hidden)]
    pub fn get_value<U>(&self, a: U) -> U {
        a
    }
}
