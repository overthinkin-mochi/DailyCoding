fn is_sum(k: i32, arr: &[i32]) -> bool {
    for i in 0..arr.len() - 1 {
        for j in i + 1..arr.len() {
            if arr[j] + arr[i] == k {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let test1 = [10, 15, 3, 7];
    let test2 = [9, 16, 4, 8];

    println!("{}", is_sum(13, &test1));
    println!("{}", is_sum(13, &test2));
}
