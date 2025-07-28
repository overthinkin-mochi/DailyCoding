/*
# Good morning! Here's your coding interview problem for today

This problem was asked by Uber.

Given an array of integers, return a new array such that each element at index i
of the new array is the product of all the numbers in the original array except
the one at i.

For example, if our input was [1, 2, 3, 4, 5], the expected
output would be [120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected
output would be [2, 3, 6].

Follow-up: what if you can't use division?

*/

fn array_colapse_with_division<const N: usize>(arr: &mut [i32; N]) {
    let product: i32 = arr.iter().product();
    arr.iter_mut().for_each(|x| *x = product / *x);
}

/*
fn arrayColapse(k: i32, arr: &[i32]) -> &[i32] {
    let mut product: i32 = 1;

    for i in 0..arr.len(){
        if ()
    }
}
*/

fn main() {
    let mut test1: [i32; 5] = [1, 2, 3, 4, 5];
    array_colapse_with_division(&mut test1);

    print!("testing with division [1, 2, 3, 4, 5]: {:#?}", test1);
}
