#![feature(async_await)]
#![feature(await_macro)]
#![feature(futures_api)]
#![feature(pin)]

use std::time::{Duration, Instant};
use tokio::timer::Delay;

use back_to_the_future::futures_await;
use back_to_the_future::BoxIntoFutures;
use back_to_the_future::IntoFutures;

#[test]
fn box_into_futures() {
	let f = async {
		futures_await!(Delay::new(Instant::now() + Duration::new(0, 10))).unwrap();
		Ok(())
	};
	tokio::run(f.box_into_futures());
}

#[test]
fn into_futures() {
	let f = async {
		futures_await!(Delay::new(Instant::now() + Duration::new(0, 10))).unwrap();
		Ok(())
	};
	tokio::run(Box::pinned(f).into_futures());
}
