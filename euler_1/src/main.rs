fn main() {
    let max = 1000;
    let mut sum = 0;

    for i in 1..max {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("sum is {}", sum);
}
