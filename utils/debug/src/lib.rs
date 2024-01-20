#[macro_export]
macro_rules! time {
    ($x:expr) => {{
        let start = std::time::Instant::now();
        let result = $x; // Execute `x`
        let elapsed = start.elapsed();
        println!("Executed in: {:?}", elapsed);
        result
    }};
}
