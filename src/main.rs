fn main() {
    print_result();
    print_result_with_label();
    conditional_loops_with_while();
    looping_through_collection_with_for();
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

// looping through a collection with for
fn looping_through_collection_with_for(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // this is a while loop in disguise
    //* this is more error prone and also slower than the for loop below
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // this is the same as above
    //* this is safer and faster than the while loop above
    //* this is also more concise
    for element in a.iter(){
        println!("the value is: {}", element);
    }

    // this is the same as above
    //? this generates a range from 1 to 3, then reverses it
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
