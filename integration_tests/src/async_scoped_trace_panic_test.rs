use googletest::prelude::*;
use googletest::InstrumentedFuture;
fn sync_function_with_trace() {
    scoped_trace!("Sync trace message");
    panic!("Intentional panic in sync function");
}

#[tokio::main]
async fn main() {
    // Initialize Google Test to install panic hook
    googletest::internal::test_outcome::TestOutcome::init_current_test_outcome();

    let fut = async {
        scoped_trace!("Outer async trace message");

        // Yield to verify async traces are preserved across yields
        tokio::task::yield_now().await;

        // Call sync function that adds a trace and panics
        sync_function_with_trace();
    };

    // Wrap in InstrumentedFuture to preserve traces across yields
    let fut = InstrumentedFuture::new(fut);

    fut.await;
}
