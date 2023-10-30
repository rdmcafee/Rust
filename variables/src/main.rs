fn main() {
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The vlaue of x is: {x}");

    println!("\nNow lets look at shadowing variables\n");

    let y = 5;

    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y in the outer scope is: {y}");

    println!("\nNow lets look at tuples...\n");

    let tup = (500, 6.4, 1);
    let (a, b, c) = tup;
    let new_a = tup.0;
    println!("The value of b is: {b}");
    println!("Now using indexes to directly access the tuple...The value of a is: {new_a}");

    //Note: tuples can contain elements of different types...arrays must contain all same type

    
}
