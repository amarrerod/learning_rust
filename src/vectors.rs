
pub fn create_and_return_vector(size: i32) -> Vec<i32> {
    let mut my_new_vector : Vec<i32> = vec![];
    for i in 0..size {
        my_new_vector.push(i);
    }
    return my_new_vector;
}