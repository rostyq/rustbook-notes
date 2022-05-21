const APPROX_PI: f32 = 3.14;

fn main() {
    println!("# Mutability");
    let mut x = 5;  // mutable value
    println!("The value of `x` is: {}", x);
    x = 6;  // mutation
    println!("The value of `x` is: {}", x);

    println!("# Constants");
    println!("The value of constant `APPROX_PI` is {}", APPROX_PI);

    println!("# Shadowing");
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of `x` in the inner scope is: {}", x);
    }

    println!("The value of `x` is: {}", x);

    let spaces = "____";  // shadowed variables cannot be mutable
    println!("`spaces`: {}", spaces);
    let spaces = spaces.len();
    println!("`spaces`: {}", spaces);

}
