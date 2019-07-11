use chapter19::HelloMacro;
use hello_macro::{hello_trace, HelloMacro};

#[derive(HelloMacro)]
struct MrsPancakes;

#[hello_trace]
fn add(left: i32, right: i32) -> i32 {
    left + right
}

// #[hello_trace]
fn add_tuple((left, right): (i32, i32)) -> i32 {
    left + right
}

// #[hello_trace]
fn add_debug(left: i32, right: i32) -> i32 {
    print!("[debug add]");
    print!(" left: {:?}", left);
    print!(" right: {:?}", right);
    println!("");
    let __debug_start = std::time::Instant::now();
    let __debug_returns = { left + right };
    let __debug_end = std::time::Instant::now();
    println!(
        "[debug add] return: {:?} (took {:?})",
        __debug_returns,
        __debug_end - __debug_start
    );
    __debug_returns
}

#[hello_trace]
fn naive_fib(nth: u32) -> i32 {
    if nth <= 1 {
        1
    } else {
        naive_fib(nth - 1) + naive_fib(nth - 2)
    }
}

fn main() {
    // MrsPancakes::hello_macro();
    // add(12, 24);
    // add_tuple((12, 24));
    // add_debug(12, 24);
    println!("{:?}", naive_fib(30));
}
