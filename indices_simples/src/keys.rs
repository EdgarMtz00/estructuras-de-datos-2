use file_storage::file_storage::Deserializable;
use std::fmt;
use crate::get_input;
// TODO: Utilizar macros para automatizar la implementacion de Display y Deserializable

pub struct Key {
    pub id: u32,
    key: String,
}

impl Key {
    pub fn new(id: u32) -> Key {
        println!("Ingrese la clave: ");
        let key = get_input();

        Key{
            id,
            key,
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clave: {}\n", self.key)
    }
}

impl Deserializable for Key {
    fn deserialize(attributes: Vec<String>) -> Self {
        let id = attributes[0]
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let key = attributes[1]
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .collect::<String>();

        Key { id, key }
    }

    fn serialize(&self) -> String {
        format!("{:^5}|{:^5}\n", self.id, self.key)
    }
}
