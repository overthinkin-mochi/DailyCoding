/*
The drawing below gives an idea of how to cut a given "true" rectangle into squares ("true" rectangle meaning that the two dimensions are different).

Can you translate this drawing into an algorithm?

You will be given two dimensions

    a positive integer length
    a positive integer width

You will return a collection or a string (depending on the language; Shell bash, PowerShell, Pascal and Fortran return a string) with the size of each of the squares.
Examples in general form:

(depending on the language)

  sqInRect(5, 3) should return [3, 2, 1, 1]
  sqInRect(3, 5) should return [3, 2, 1, 1]

  You can see examples for your language in **"SAMPLE TESTS".**

Notes:

    lng == wdth as a starting case would be an entirely different problem and the drawing is planned to be interpreted with lng != wdth. (See kata, Square into Squares. Protect trees! http://www.codewars.com/kata/54eb33e5bc1a25440d000891 for this problem).

    When the initial parameters are so that lng == wdth, the solution [lng] would be the most obvious but not in the spirit of this kata so, in that case, return None/nil/null/Nothing or return {} with C++, [] with Perl, Raku.

    In that case the returned structure of C will have its sz component equal to 0.

    Return the string "nil" with Bash, PowerShell, Pascal and Fortran.
ify the input array in-place.

*/
use std::cmp;

fn sq_in_rect_recursive(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    //base case
    if lng == wdth {
        return None;
    }

    let large = cmp::max(lng, wdth);
    let small = cmp::min(lng, wdth);

    match sq_in_rect(small, large - small) {
        None => Some(vec![small, small]),
        Some(x) => Some([vec![small], x].concat()),
    }
}

fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth {
        return None;
    }

    let mut cuts = Vec::new();
    let mut x = wdth;
    let mut y = lng;

    while x > 0 && y > 0 {
        if y > x || x == y {
            cuts.push(x);
            y -= x;
        } else if x > y {
            cuts.push(y);
            x -= y;
        }
    }
    Some(cuts)
}

fn main() {
    println!("slices of rectangle: {:?}", sq_in_rect_recursive(3, 7));
}
