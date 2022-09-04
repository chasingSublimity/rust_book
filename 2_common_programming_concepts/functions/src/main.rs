fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("y: {y}");
    println!("{:?}", five());

    print_labeled_measurements(5, 'h');
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}