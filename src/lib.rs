use futures::{
    ready,
    stream::Stream,
    task::{Context, Poll},
};
use std::{pin::Pin, time::Duration};
use tokio::time::{self, Interval};

/// Creates new `NumStream` that yields values at an interval of `period` that
/// continuously increment by `increment` where the first value yielded is `initial`.
///
/// A `NumStream` will yield values indefinitely. At any time, the `NumStream` value
/// can be dropped, canceling the stream.
///
/// # Panics
///
/// This function panics if `period` is zero.
///
/// # Examples
///
/// ```
/// use futures::stream::StreamExt;
/// use num_stream::num_stream;
/// use std::time::Duration;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() {
///     let mut nums = num_stream(0, 3, Duration::from_millis(500));
///     loop {
///         println!("Got: {:?}", nums.next().await);
///     }
/// }
/// ```
pub fn num_stream(initial: u64, increment: u64, period: Duration) -> NumStream {
    assert!(period > Duration::new(0, 0), "`period` must be non-zero.");
    NumStream {
        num: initial,
        increment,
        timer: time::interval(period),
    }
}

/// Stream returned by [`num_stream`](fn.num_stream.html)
#[derive(Debug)]
pub struct NumStream {
    /// The next value that will be yielded by `NumStream`
    num: u64,

    /// How much `num` is incremented by after yielding
    increment: u64,

    /// The interval between values yielded by `NumStream`
    timer: Interval,
}

impl Stream for NumStream {
    type Item = u64;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match ready!(Pin::new(&mut self.timer).poll_next(cx)) {
            Some(_) => {
                let retval = self.num;
                self.num = self.num.wrapping_add(self.increment);
                return Poll::Ready(Some(retval));
            }
            None => return Poll::Ready(None),
        }
    }
}
