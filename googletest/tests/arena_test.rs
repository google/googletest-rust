use googletest::{description::Description, matcher::MatcherResult, prelude::*};
use std::{fmt::Debug, marker::PhantomData, ops::Deref};

#[derive(Debug)]
struct ViewProxy<'a, T: ?Sized> {
    value: &'a T,
}

impl<'a, T> Clone for ViewProxy<'a, T> {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}

#[derive(Debug, PartialEq)]
struct Strukt {
    a_field: i32,
    a_string: String,
}

type StruktView<'a> = ViewProxy<'a, Strukt>;

impl<'a> ViewProxy<'a, Strukt> {
    fn get_a_field(&self) -> ViewProxy<'_, i32> {
        ViewProxy { value: &self.value.a_field }
    }

    #[allow(unused)]
    fn get_a_string(&self) -> ViewProxy<'_, str> {
        ViewProxy { value: &self.value.a_string }
    }
}

impl<'data, T: ?Sized> Deref for ViewProxy<'data, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'data, T> AsRef<T> for ViewProxy<'data, T> {
    fn as_ref(&self) -> &T {
        self.value
    }
}

#[allow(unused)]
fn property_matcher<
    OuterT: Debug + Clone,
    InnerT: Debug + Clone,
    InnerRefT: Deref<Target = InnerT> + Clone,
    MatcherT: Matcher<ActualT = InnerT>,
>(
    extractor: impl Fn(OuterT) -> InnerRefT,
    property_desc: &'static str,
    inner: MatcherT,
) -> impl Matcher<ActualT = OuterT> {
    PropertyMatcher { extractor, property_desc, inner, phantom: Default::default() }
}

struct PropertyMatcher<OuterT, ExtractorT, MatcherT> {
    extractor: ExtractorT,
    property_desc: &'static str,
    inner: MatcherT,
    phantom: PhantomData<OuterT>,
}

impl<InnerT, InnerRefT, OuterT, ExtractorT, MatcherT> Matcher
    for PropertyMatcher<OuterT, ExtractorT, MatcherT>
where
    InnerT: Debug + Clone,
    InnerRefT: Deref<Target = InnerT> + Clone,
    OuterT: Debug + Clone,
    ExtractorT: Fn(OuterT) -> InnerRefT,
    MatcherT: Matcher<ActualT = InnerT>,
{
    type ActualT = OuterT;

    fn matches<ActualRefT: Deref<Target = Self::ActualT>>(
        &self,
        actual: ActualRefT,
    ) -> MatcherResult {
        self.inner.matches((self.extractor)(actual.deref().clone()))
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        format!(
            "has property `{}`, which {}",
            self.property_desc,
            self.inner.describe(matcher_result)
        )
        .into()
    }

    fn explain_match<ActualRefT: Deref<Target = Self::ActualT>>(
        &self,
        actual: ActualRefT,
    ) -> Description {
        let actual_inner = (self.extractor)(actual.deref().clone());
        format!(
            "whose property `{}` is `{:#?}`, {}",
            self.property_desc,
            actual_inner.clone().deref(),
            self.inner.explain_match(actual_inner)
        )
        .into()
    }
}

fn has_a_field<'actual>(
    inner: impl Matcher<ActualT = i32>,
) -> impl Matcher<ActualT = StruktView<'actual>> {
    struct HasAFieldMatcher<'actual, InnerMatcherT: Matcher<ActualT = i32>> {
        inner: InnerMatcherT,
        phantom: PhantomData<&'actual ()>,
    }

    impl<'actual, InnerMatcherT: Matcher<ActualT = i32>> Matcher
        for HasAFieldMatcher<'actual, InnerMatcherT>
    {
        type ActualT = StruktView<'actual>;

        fn matches<ActualRefT: Deref<Target = Self::ActualT> + Clone>(
            &self,
            actual: ActualRefT,
        ) -> MatcherResult {
            self.inner.matches(actual.get_a_field())
        }

        fn describe(&self, _: MatcherResult) -> Description {
            todo!()
        }
    }

    HasAFieldMatcher { inner, phantom: Default::default() }
}

#[test]
fn check() -> Result<()> {
    let arena = vec![Strukt { a_field: 33, a_string: "something".to_string() }];
    let holder = ViewProxy { value: &arena[0] };

    // Problem: This does not compile because get_a_field() constraints the lifetime of its
    // returned ViewProxy to that of the parameter v, which is local to the closure.
    // verify_that!(holder, property_matcher(|v: StruktView| v.get_a_field(), "a_field", eq(33)))?;
    verify_that!(holder, has_a_field(eq(33)))?;
    Ok(())
}
