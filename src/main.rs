mod ternary;

fn main() {
    println!("Hello, world!");
    println!("{}, {}!", "Hello", "world");
    println!("{0}, {1}!", "Hello", "world");
    println!("{greeting}, {name}!", greeting = "Hello", name = "world");
    println!("{:?}", [1, 2, 3]);
    println!("{:#?}", [1, 2, 3]);

    let x = format!("{}, {}!", "Hello", "world"); // Cadena formateada
    println!("{}", x);

    ternary::is_under();
    println!(
        "Is under 18 with 15 years: {}",
        ternary::is_under_with_args(15)
    );
    println!(
        "Is under 18 with 20 years: {}",
        ternary::is_under_with_args(20)
    );
}
