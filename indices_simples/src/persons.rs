use file_storage::file_storage::Deserializable;
use std::fmt;
use crate::get_input;

pub struct Person {
    pub id: u32,
    nombre: String,
    apellidos: String,
    edad: u32,
    peso: u32,
    altura: u32,
}

impl Person {
    pub fn new(id: u32) -> Person{
        println!("Ingrese el nombre: ");
        let nombre = get_input();

        println!("Ingrese los apellidos: ");
        let apellidos = get_input();

        println!("Ingrese la edad: ");
        let edad = get_input().parse::<u32>().unwrap_or_else(|_| {
            println!("\nEdad no valida... Sera asignada a 0 pero puede ser modificada");
            0
        });

        println!("Ingrese el peso: ");
        let peso = get_input().parse::<u32>().unwrap_or_else(|_| {
            println!("\nPeso no valido... Sera asignado a 0 pero puede ser modificada");
            0
        });

        println!("Ingrese la altura: ");
        let altura = get_input().parse::<u32>().unwrap_or_else(|_| {
            println!("\nAltura no valida... Sera asignada a 0 pero puede ser modificada");
            0
        });

        Person {
            id,
            nombre,
            apellidos,
            edad,
            peso,
            altura,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.-\n\tNombre: {}\n\tApellidos: {}\n\tEdad: {}\n\tPeso: {}\n\tAltura: {}\n",
            self.id, self.nombre, self.apellidos, self.edad, self.peso, self.altura
        )
    }
}

impl Deserializable for Person {
    fn deserialize(attributes: Vec<String>) -> Self {
        let id = attributes[0]
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let nombre = attributes[1]
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .collect::<String>();
        let apellidos = attributes[2]
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .collect::<String>();
        let edad = attributes[3]
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let peso = attributes[4]
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let altura = attributes[5]
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        Person {
            id,
            nombre,
            apellidos,
            edad,
            peso,
            altura,
        }
    }

    fn serialize(&self) -> String {
        format!(
            "{:^5}|{:^25}|{:^25}|{:^3}|{:^5}|{:^5}\n",
            self.id, self.nombre, self.apellidos, self.edad, self.peso, self.altura
        )
    }
}
