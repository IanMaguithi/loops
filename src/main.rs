fn main() {
    print_result();
    print_result_with_label();
    conditional_loops_with_while();
}

// returning values from loops
fn print_result(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

// loop labels to disambiguate between multiple loops
fn print_result_with_label(){
    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining =10;

        'counting_down: loop{
            println!("remaining = {}", remaining);
            // if this wasn't labelled, it would break the inner loop
            if remaining == 9{
                break 'counting_down;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

// conditional loops with while
fn conditional_loops_with_while(){
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
