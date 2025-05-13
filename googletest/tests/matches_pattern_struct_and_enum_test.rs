// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use googletest::prelude::*;
use googletest::Result;

#[test]
fn matches_struct_with_a_method_taking_enum_value_parameter_followed_by_field() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }
    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(&AStruct { get_field(AnEnum::AVariant): eq(1), another_field: eq(2), .. })
    )
}

#[test]
fn matches_struct_with_a_method_taking_enum_value_param_ret_ref_followed_by_field() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }
    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(&AStruct {
            get_field_ref(AnEnum::AVariant): eq(&1),
            another_field: eq(2),
            ..
        })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }
    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(&AStruct {
            another_field: eq(2),
            get_field(AnEnum::AVariant): eq(1),
            ..
        })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param_ret_ref() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }
    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(&AStruct {
            another_field: eq(2),
            get_field_ref(AnEnum::AVariant): eq(&1),
            ..
        })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param_followed_by_field(
) -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }
    let actual = AStruct { a_field: 1, another_field: 2, a_third_field: 3 };

    verify_that!(
        actual,
        matches_pattern!(&AStruct {
            another_field: eq(2),
            get_field(AnEnum::AVariant): eq(1),
            a_third_field: eq(3),
            ..
        })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param_ret_ref_followed_by_field(
) -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }
    let actual = AStruct { a_field: 1, another_field: 2, a_third_field: 3 };

    verify_that!(
        actual,
        matches_pattern!(&AStruct {
            another_field: eq(2),
            get_field_ref(AnEnum::AVariant): eq(&1),
            a_third_field: eq(3),
            ..
        })
    )
}

#[test]
fn matches_struct_with_a_method_returning_reference_taking_enum_value_parameter() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }
    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(&AStruct { get_field_ref(AnEnum::AVariant): eq(&1) }))
}

#[test]
fn matches_struct_with_a_method_taking_enum_value_parameter() -> Result<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_a_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }
    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(&AStruct { get_a_field(AnEnum::AVariant): eq(1) }))
}
