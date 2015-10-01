fn is_palindrome(num: i32) -> bool {
    let num_str = num.to_string();
    let rev_num_str: String = num_str.chars().rev().collect();

    num_str == rev_num_str
}

fn main() {
    let mut max = 0;
    for i in (99..999).rev() {
        for j in (99..999).rev() {
            let m = i * j;
            if is_palindrome(m) {
                if max < m {
                    max = m;
                }
            }
        }
    }
    println!("Palindrome: {}", max);
}
