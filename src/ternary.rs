pub fn is_under() {
    let age = 13;

    let is_under_18 = if age < 18 { true } else { false };

    println!("Is under 18: {}", is_under_18);
}

pub fn is_under_with_args(age: i32) -> bool {
    let is_under: bool = if age < 18 { true } else { false };
    is_under
}
