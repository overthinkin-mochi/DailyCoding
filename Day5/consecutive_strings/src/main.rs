fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len() < k {
        return "".to_owned();
    }

    let mut longest: String = String::new();

    for i in 0..=strarr.len() - k {
        let mut current_string = String::new();

        for j in i..i + k {
            current_string += strarr[j]
        }
        println!("{}", current_string);
        if current_string.len() > longest.len() {
            longest = current_string;
        }
    }
    longest
}

fn main() {
    let vec1 = vec!["some", "things", "are", "meant", "to", "be", "broken"];

    println!("{:?}", longest_consec(vec1, 3));
}
