use std::env::args;

fn main() {
    match args()
        .nth(1)
        .expect("first positional argument is required")
        .as_ref()
    {
        "convert" => convert(),
        "fib" => fib(),
        program => panic!("unknown program `{}`", program),
    }
}

//#region convert
fn convert() {
    let help = "-f <deg> to convert from F; -c <deg> to convert from C";
    match args().nth(2).expect(help).as_ref() {
        "-f" => from_f(),
        "-c" => from_c(),
        _ => panic!(help),
    };
}

fn convert_parse() -> f32 {
    let err = "please pass a number";
    let str = args().nth(3).expect(err);
    str.parse().expect(err)
}
fn from_f() {
    let f = convert_parse();
    let c = (f - 32.0) * 5.0 / 9.0;
    println!("{} F is {:.2} C", f, c);
}
fn from_c() {
    let c = convert_parse();
    let f = c * 9.0 / 5.0 + 32.0;
    println!("{} C is {:.2} F", c, f);
}
//#endregion

//#region fib
fn fib() {
    let nth = fib_parse();

    let fib = match nth {
        1 | 2 => 1,
        _ => {
            let mut cache: (i128, i128) = (1, 1);
            for _ in 2..nth {
                cache = (cache.1, cache.0 + cache.1)
            }

            cache.1
        }
    };
    println!("the nth fib is {} where n={}", fib, nth);
}
fn fib_parse() -> i32 {
    let err = "please pass a positive integer";
    let str = args().nth(2).expect(err);
    let i = str.parse().expect(err);
    if i < 1 {
        panic!(err)
    }
    i
}
//#endregion
