fn main() {
    println!("Hello, world!");
    println!("The area of 5, 6 {}", area(5, 6));
    println!(
        "{name} likes to play {instrument}",
        name = "Man",
        instrument = "mayonese"
    );
    println!("Binary: {:b}, Hex: {:x}, Octal {:o}", 10, 10, 10);
    println!("var_dump in php is {:?}", ["this", "is", "array", "?"]);
    // variables hold primitives or pointers
    // variables are immutable
    // mod to change value of variable
    let mut z = 32;
    z += 1;
    const XD: i32 = 69;
    println!("{}", XD);
    let (my_name, my_age) = ("hello", 17);
    println!("{:?}", (my_age, my_name));
    let mut string = String::from("Hello");
    string.push_str(", World!");
    println!("{}", string);
    string.is_empty();
    let shit = ("Hi", "this", "is", "my", 1, "touple", '!');
    // arrays are fixed FUCK ME THIS IS A RETARDED LANGUAGE
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", numbers[0]);
    numbers[0] = 0;
    println!("{}", numbers.len());
    println!("{}", std::mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[0..3];
    println!("{:?}", slice);
    // vectors are les retarded arrays
    let mut vectors: Vec<i32> = vec![1, 2, 3, 4, 5];
    vectors.push(6);
    vectors.len();
    vectors.last();
    // vec.iter nie pozwala na zmiany w wartościach. vec.iter_mut pozwala na zmiany wartości
    for elem in vectors.iter_mut() {
        *elem *= 2;
    }
    let mut x = 0;
    while x < 5 {
        x += 1;
    }
    // we can use && || !=
    let mut z = 5;
    let mut x = 6;
    z >> 2;
    println!("{}", z);
    // loop is an infinite loop
    loop {
        break;
    }
    // while loop
    while 2 + 2 == 4 {
        break;
    }
    for num in 0..10000 {
        break;
    }
    // pointers
    let x = vec![1, 2, 3];
    x.push(4);
    let y = &x;
    // traditional struct
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }
    let mut c = Color(255, 0, 255);
    c.red = 200;
    println!("rgb({}, {}, {})", c.red, c.blue, c.green);
    struct X(u8, u8, u8);
    let mut p = X(255, 2, 0);
    struct Person {
        first_name: String,
        last_name: String,
    }
    impl Person {
        fn new(first: &str, last: &str) -> Person {
            Person {
                first_name: first.to_string(),
                last_name: last.to_string(),
            }
        }
        fn change_last_name(&mut self, name: &str) {
            self.last_name = name.to_string();
        }
    }
}

fn area(x: i32, y: i32) -> i32 {
    return x * y;
}
