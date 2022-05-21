fn main() {
    let guess: u32 = "42".parse().expect("NaN!");
    println!("`guess`: {}", guess);

    let x = 2.0;  // f64
    println!("`x`: {}", x);
    let y: f32 = 3.0;  // f32
    println!("`y`: {}", y);

    println!("# addition");
    let result = 5 + 10;
    println!("5 + 10 = {}", result);

    println!("# subtraction");
    let result = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", result);

    println!("# multiplication");
    let result = 4 * 30;
    println!("4 * 30 = {}", result);

    println!("# division");
    let result = 56.7 / 32.2;
    println!("quotient: 56.7 / 32.2 = {}", result);
    let result = 4 / 3;
    println!("floored: 4 / 3 = {}", result);

    println!("# remainder");
    let result = 43 % 5;
    println!("43 % 5 = {}", result);

    println!("# boolean");
    let t = true;
    let f: bool = false;
    println!("t = {}, f = {}", t, f);

    println!("# char");
    let c1 = 'c';
    let c2 = 'ї';
    let c3 = '😻';
    println!("c1 = '{}', c2 = '{}', c3 = '{}'", c1, c2, c3);

    println!("# tuple");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);
    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);
    println!("tup.2 = {}", tup.2);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);

    println!("# array");
    let arr = [1, 2, 3, 4, 5];
    println!("arr = {:?}", arr);
    println!("first = arr[0] = {}", arr[0]);
    println!("last = {}", arr[arr.len() - 1]);

    let arr = [3; 5];
    println!("arr = {:?}", arr);

    println!("# invalid array element access");

    let arr = [1, 2, 3, 4, 5];

    let index: usize = "4"
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
