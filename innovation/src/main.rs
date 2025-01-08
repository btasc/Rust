fn fibonacci(fib_len: usize) -> Vec<i32> {
    assert!(fib_len >= 2, "N must be at least 2");

    let mut sequence = vec![0, 1];

    for i in 2..fib_len {
        sequence.push(sequence[i - 1] + sequence[i - 2]);
    }

    return sequence;
}

fn main() { 
    let fib_len: i32 = 1000;
    let fib_sequence: Vec<i32> = fibonacci(fib_len as usize);


    println!("{:?}", fib_sequence);
}