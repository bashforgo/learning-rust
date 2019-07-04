type Value = i32;

fn split_first(values: Vec<Value>) -> (Value, Vec<Value>) {
    let mut first = true;
    let mut result = (0, vec![]);

    for v in values.into_iter() {
        if first {
            first = false;
            result.0 = v
        } else {
            result.1.push(v);
        }
    }

    result
}

fn partition<Predicate>(values: Vec<Value>, pred: Predicate) -> (Vec<Value>, Vec<Value>)
where
    Predicate: Fn(&Value) -> bool,
{
    let mut result = (vec![], vec![]);

    for v in values {
        let result = if pred(&v) {
            &mut result.0
        } else {
            &mut result.1
        };
        result.push(v);
    }

    result
}

fn quick_sort(values: Vec<Value>) -> Vec<Value> {
    if values.len() <= 1 {
        values
    } else {
        let (pivot, rest) = split_first(values);
        let (left, right) = partition(rest, |v| *v < pivot);

        let mut result: Vec<Value> = quick_sort(left);
        result.push(pivot);
        result.append(&mut quick_sort(right));

        result
    }
}

fn main() {
    let v = vec![
        235, 25, 253, 23, 414, 14, 12, 431, 24, 13, 543, 63, 647, 568, 67, 969, 85, 47535, 6243,
    ];
    let s = quick_sort(v);
    println!("{:?}", s);
}
