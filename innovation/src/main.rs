
enum Charlie {
    Yeah = 82
}


fn main() {
    let _arr: [i32; 5] = [1, 4, 2, 21, -5];

    const YEAH_TIME: i32 = Charlie::Yeah as i32;

    println!("{:?}", YEAH_TIME);
}