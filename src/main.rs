mod ternary;
mod vectors;
mod impls;

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

    println!("Creating a new array of size 10: {:#?}", vectors::create_and_return_vector(10));
    
    let player_1 = impls::Player {
        first_name: "Rafael".to_string(),
        last_name: "Nadal".to_string(),
        age: 35
    };
    println!("Player: {} with {} years", player_1.full_name(), player_1.age);
}



