struct Fibonacci {
    curr: u32,
    next: u32
}

// implement iterator for fibonacci
impl Iterator for Fibonacci {
    type Item = u32;

    // the Iterator trait only requires next method to be defined
    // return type is Option<T>, None is returned when iterator is over
    // otherwise next value is returned wrapped in Some

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Some is always returned, infinite value generator
        Some(self.curr)
    }
}

// returns a fibonacci sequence generator
// fn fibonacci() -> Fibonacci {
//     Fibonacci {curr: 1, next: 1}
// }

fn main() {

    let max = 4000000;
    let fib = Fibonacci {curr: 1, next: 1};
    let mut sum = 0;

    // get n items
    for i in fib {
        if i >= max {
            break;
        }
        if i % 2 == 0 {
            sum += i;
        }
    }
    println!("Sum: {}", sum);
}
