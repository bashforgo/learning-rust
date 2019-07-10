use chapter19::HelloMacro;
use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct MrsPancakes;

fn main() {
    MrsPancakes::hello_macro();
}
