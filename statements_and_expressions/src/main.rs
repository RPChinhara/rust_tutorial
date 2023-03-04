fn five() -> u32 {
    5
}

fn main() {
    let y = {
        let x = 4;
        x + 2
    };

    println!("The value of y is {y}");

    let z = five();
    println!("The value of z is {z}")
}