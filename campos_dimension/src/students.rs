use std::io::{Error, ErrorKind};
use campos_dimension::file_storage::Deserializable;
use std::fmt;
use std::str;
use super::get_input;

pub struct Student {
    pub id:u32, 
    pub nombre: String,
    pub apellidos: String,
    pub codigo : String,
    pub semestre: u32,
    pub promedio: u32,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.-\n Nombre: {}\n Apellido: {}\n Codigo: {}\n Semestre: {}\n Promedio: {}\n", self.id, self.nombre, self.apellidos, self.codigo, self.semestre, self.promedio)
    }
}

impl Deserializable for Student{
    fn deserialize(attributes: Vec<String>) -> Self{
        let id = attributes[0].chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();
        let nombre= attributes[1].chars().collect::<String>().trim().to_string();
        let apellidos = attributes[2].chars().collect::<String>().trim().to_string();
        let codigo  = attributes[3].chars().collect::<String>().trim().to_string();
        let semestre = attributes[4].chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();
        let promedio = attributes[5].chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();

        Student{
            id,
            nombre,
            apellidos,
            codigo,
            semestre,
            promedio
        }
    }

    fn serialize(&self) -> String{
        format!("{}{:^5}{}{:^12}{}{:^12}{}{:^8}{}{:^2}{}{:^3}\n", 
        str::from_utf8(&[5]).unwrap(), self.id, 
        str::from_utf8(&[12]).unwrap(), self.nombre,
        str::from_utf8(&[12]).unwrap(), self.apellidos,
        str::from_utf8(&[8]).unwrap(), self.codigo,
        str::from_utf8(&[2]).unwrap(), self.semestre,
        str::from_utf8(&[3]).unwrap(), self.promedio
        )
    }
}

impl Student {
    pub fn new(id:u32) -> Student {
        println!("Ingrese el nombre del estudiante: ");
        let nombre = get_input();
        println!("Ingrese los apellidos del estudiante: ");
        let apellidos = get_input();
        println!("Ingrese el codigo del estudiante: ");
        let codigo = get_input();
        println!("Ingrese el semestre del estudiante: ");
        let semestre = get_input().parse::<u32>().unwrap();
        println!("Ingrese el promedio del estudiante: ");
        let promedio = get_input().parse::<u32>().unwrap();
        Student{
            id,
            nombre,
            apellidos,
            codigo,
            semestre,
            promedio,
        }
    }
}
