
pub struct Player {
    pub first_name : String,
    pub last_name : String,
    pub age: i32,
}

impl Player {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

}