/*
Welcome. In this kata, you are asked to square every digit of a number and concatenate them.

For example, if we run 9119 through the function, 811181 will come out, because 92 is 81 and 12 is 1. (81-1-1-81)

Example #2: An input of 765 will/should return 493625 because 72 is 49, 62 is 36, and 52 is 25. (49-36-25)

Note: The function accepts an integer and returns an integer.

Happy Coding!

*/

fn square_digits(num: u64) -> u64 {
    let numstring: String = num.to_string();
    let mut output = String::new();

    for letter in numstring.chars() {
        let digit: u64 = letter.to_digit(10).unwrap() as u64;
        let squared = digit.pow(2);
        output.push_str(&squared.to_string());
    }

    return output.parse().unwrap();
}

fn main() {
    let test1: u64 = 765;
    println!("{}", square_digits(test1));
}
