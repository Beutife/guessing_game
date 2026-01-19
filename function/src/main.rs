fn main() {
    println!("Hello, world!");

    another_function(6);

    print_labeled_measurement(5, 'h');

    // Moved this logic into main so it actually runs
    let x = five();
    println!("The value of x is: {x}");

    let y = {
        let x = 3;
        x + 1 // No semicolon here: this is an expression that returns 4
    };
    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("Another function");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5 // Removed the semicolon so it returns the value 5
}