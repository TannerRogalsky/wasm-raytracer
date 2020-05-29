fn main() {
    let pool = futures::executor::ThreadPool::new().unwrap();
    pool.spawn_ok(async {
        println!("TEST");
    });
}
