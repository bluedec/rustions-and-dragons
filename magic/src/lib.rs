pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Human {
    name: String,
    health: i32,
    class: Class,
}
pub struct Class {
    name: String,
    level: i32,
}

pub struct Weapon {
    name: String,
    level: i32,  
    damage: i32,
    class: Class,
    min_lvl: i32, 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
