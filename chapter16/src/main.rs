use std::sync::mpsc;
use std::thread;

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

const SPAWN_LEN_LIMIT: usize = 10_000;
fn quick_sort(values: Vec<Value>) -> Vec<Value> {
    if values.len() <= 1 {
        values
    } else {
        let (pivot, rest) = split_first(values);
        let (left, right) = partition(rest, |v| *v < pivot);

        let (left_tx, left_rx) = mpsc::channel();
        let (right_tx, right_rx) = mpsc::channel();

        if left.len() < SPAWN_LEN_LIMIT {
            left_tx.send(quick_sort(left)).unwrap();
        } else {
            thread::spawn(move || {
                left_tx.send(quick_sort(left)).unwrap();
            });
        }

        if right.len() < SPAWN_LEN_LIMIT {
            right_tx.send(quick_sort(right)).unwrap();
        } else {
            thread::spawn(move || {
                right_tx.send(quick_sort(right)).unwrap();
            });
        }

        let (left_result, mut right_result) = (left_rx.recv().unwrap(), right_rx.recv().unwrap());

        let mut result: Vec<Value> = left_result;
        result.push(pivot);
        result.append(&mut right_result);

        result
    }
}

fn main() {
    use rand::Rng;
    use std::time;

    let mut rng = rand::thread_rng();

    println!("creating vec");
    let mut v = vec![];
    for _ in 0..10_000_000 {
        v.push(rng.gen());
    }

    println!("sorting");
    let start = time::Instant::now();
    let _ = quick_sort(v);
    let end = time::Instant::now();
    println!("done, took {:?}", end - start);
}
