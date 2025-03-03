#![cfg(test)]

use futures::{executor::block_on, join};
use std::thread;

fn download(_url: &str) {
    // ...
}

#[test]
// ANCHOR: get_two_sites
fn get_two_sites() {
    // 仕事を行うために、2つのスレッドを生成します。
    let thread_one = thread::spawn(|| download("https://www.foo.com"));
    let thread_two = thread::spawn(|| download("https://www.bar.com"));

    // 両方のスレッドが完了するのを待ちます。
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}
// ANCHOR_END: get_two_sites

async fn download_async(_url: &str) {
    // ...
}

// ANCHOR: get_two_sites_async
async fn get_two_sites_async() {
    // 完了するまで実行することで、ウェブページを非同期でダウンロードする
    // 2つの異なる "future" を作成します。
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");

    // 両方の "future" を同時に完了するまで実行します。
    join!(future_one, future_two);
}
// ANCHOR_END: get_two_sites_async

#[test]
fn get_two_sites_async_test() {
    block_on(get_two_sites_async());
}
