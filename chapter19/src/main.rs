use chapter19::HelloMacro;
use hello_macro::{hello_trace, HelloMacro};

#[derive(HelloMacro)]
struct MrsPancakes;

struct TestStruct {
    struct_value: i32,
}
struct TestTuple(i32);

enum PathEnum {
    OnlyValue,
}

#[hello_trace]
fn test_trace(
    _: i32,
    ident: i32,
    TestStruct { struct_value }: TestStruct,
    TestTuple(tuple_struct): TestTuple,
    PathEnum::OnlyValue: PathEnum,
    (t0, t1): (i32, i32),
    reference: &i32,
    mut_reference: &mut i32,
) -> i32 {
    std::thread::sleep_ms(100);
    ident * struct_value * tuple_struct * t0 * t1 * *reference * *mut_reference
}

fn main() {
    MrsPancakes::hello_macro();
    test_trace(
        1,
        2,
        TestStruct { struct_value: 3 },
        TestTuple(4),
        PathEnum::OnlyValue,
        (5, 6),
        &7,
        &mut 8,
    );
}
