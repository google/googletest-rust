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
