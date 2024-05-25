fn fibonacci_sequence(limit:usize) -> Vec<u128>{
    // Creates a fibonacci sequence list until specified
    // Parameters:
    //  limit (usize): How long the fibonacci sequence will be
    // Returns:
    //  fibonacci (Vec<u128>)
    let mut fibonacci = Vec::with_capacity(limit);

    // Fills the list
    for num in 0..limit{
        // The first two values are fixed
        if num == 0{
            fibonacci.push(0);
        }
        else if num == 1 {
            fibonacci.push(1);
        }
        // From the third value you can calculate it
        else {
            let next_value = (fibonacci[num - 2] + fibonacci[num - 1]);
            fibonacci.push(next_value);
        }
    }
    return fibonacci;
}

fn lucas_sequence(limit:usize) -> Vec<u128>{
    // Creates a lucas sequence list until specified
    // Parameters:
    //  limit (usize): How long the Lucas sequence will be
    // Returns:
    //  lucas (Vec<u128>)
    let mut lucas = Vec::with_capacity(limit);

    // Fills the list
    for num in 0..limit{
        // The first two values are fixed
        if num == 0{
            lucas.push(2);
        }
        else if num == 1 {
            lucas.push(1);
        }
        // From the third value you can calculate it
        else {
            let next_value = (lucas[num - 2] + lucas[num - 1]);
            lucas.push(next_value);
        }
    }
    return lucas;
}

fn main(){
    // Declare variables
    let mut stop = false;
    let mut seq = false;
    let mut limit:usize = 0;
    let mut input1:&str = "";
    let mut input = String::new();

    // User Interface and User Input
    println!("========= Fibonacci/Lucas - Gen =========");
    println!("This program generates a fibonacci and lucas sequence until specified.");
    
    // Loop for error checking which sequence to choose
    while seq == false {

        println!("Please, specify which sequence would you like:");
        println!("1. Fibonacci Sequence");
        println!("2. Lucas Sequence");
        std::io::stdin().read_line(&mut input).unwrap();
        
        // Trims and transforms the input
        input1 = input.trim();

        // Checks for which sequence to calculate
        if input1 == "1"{
            println!("Fibonacci Sequence Chosen");
            println!("");
            seq = true;
        }
        else if input1 == "2"{
            println!("Lucas Sequence Chosen.");
            println!("");
            seq = true;
        }
    }

    // Loop for error checking the limit for the sequence
    while stop == false {
        let mut limit_line = String::new();
        println!("Please, specify the limit for the sequence:");
        std::io::stdin().read_line(&mut limit_line).unwrap();
        
        // Trims the limit line and reassings it.
        let limit_line = limit_line.trim();

        match limit_line.parse::<usize>() {
            Ok(num) => {
                limit = num;
                println!("You entered the limit: {}", limit);
                stop = true;
                // You can now use the variable `limit` as a usize
            }
            Err(_) => {
                println!("Invalid input, please enter a valid number");
                // If the variable can't be transformed to usize it returns an error.
            }
        }
    }

    if input1 == "1"{
        // Calculates and prints the Fibonacci Sequence
        let fib_sequence = fibonacci_sequence(limit);
        println!("Here's your Fibonacci sequence of {} numbers", limit);
        println!("{:?}", fib_sequence);
    }
    else if input1 == "2"{
        // Calculates and prints the Lucas Sequence
        let luc_sequence = lucas_sequence(limit);
        println!("Here's your Lucas sequence of {} numbers", limit);
        println!("{:?}", luc_sequence);
    }
    // Used to not immidiately escape from the program.
    eprintln!("Press Enter to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}