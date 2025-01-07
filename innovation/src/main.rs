


fn main() {
    fn sort<const N: usize>(arr: &mut [i32; N]) -> &[i32; N] {
        let answer: &mut [i32; N] = arr;

        for int in answer.iter_mut() {
            *int *= 2;
        }

        

        return answer;
    }

    let arr: [i32; 5] = [1, 4, 2, 21, -5];
    let sorted_arr = sort(&arr);

    println!("\n");
    println!("{:?}", sorted_arr);
}