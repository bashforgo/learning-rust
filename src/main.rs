use std::env::args;

fn main() {
    match args()
        .nth(1)
        .expect("first positional argument is required")
        .as_ref()
    {
        "convert" => convert(),
        program => panic!("unknown program `{}`", program),
    }
}

fn convert() {
    let help = "-f <deg> to convert from F; -c <deg> to convert from C";
    match args().nth(2).expect(help).as_ref() {
        "-f" => from_f(),
        "-c" => from_c(),
        _ => panic!(help),
    };
}

fn parse_third() -> f32 {
    let err = "please pass a number";
    let str = args().nth(3).expect(err);
    str.parse().expect(err)
}
fn from_f() {
    let f = parse_third();
    let c = (f - 32.0) * 5.0 / 9.0;
    println!("{} F is {:.2} C", f, c);
}
fn from_c() {
    let c = parse_third();
    let f = c * 9.0 / 5.0 + 32.0;
    println!("{} C is {:.2} F", c, f);
}
