#![no_std]
#![no_main]
extern crate ndless_handler;
extern crate alloc;
use core::future::Future;
use core::pin::Pin;
use alloc::boxed::Box;
pub struct Task<T> {
    future: Pin<Box<dyn Future<Output = T>>>,
}

impl<T> Task<T> {
    pub fn new(future: impl Future<Output = T> + 'static) -> Task<T> {
        Task {
            future: Box::pin(future),
        }
    }
}

static NUM: i32 = 5;

#[no_mangle]
fn main() {
    // Comment out this next line to make the build succeed
    doesnt_work();
    does_work();
}

fn doesnt_work() {
    Task::new(do_stuff());
    {
        let arr = [5];
        let i = 1;
        arr[i];
    }
    // OR
    {
        unimplemented!()
    }
}

fn does_work() {
    Task::new(do_more_stuff());
    {
        let arr = [5];
        let i = 1;
        arr[i];
    }
}

async fn do_stuff() -> &'static str {
    do_more_stuff().await
}

async fn do_more_stuff() -> &'static str {
    "test"
}

async fn do_stuff_working() -> &'static i32 {
    do_more_stuff_working().await
}

async fn do_more_stuff_working() -> &'static i32 {
    &NUM
}
