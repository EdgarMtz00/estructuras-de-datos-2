use std::io::{Error, ErrorKind};
use campos_dimension::file_storage::Deserializable;
use std::fmt;
use std::str;
use super::get_input;

pub struct Song {
    pub id:u32, 
    pub nombre: String,
    pub album: String,
    pub genero : String,
    pub anio: u32,
    pub duracion: u32,
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.-\n Nombre: {}\n Album: {}\n Genero: {}\n aNio: {}\n Duracion: {}\n", self.id, self.nombre, self.album, self.genero, self.anio, self.duracion)
    }
}

impl Deserializable for Song{
    fn deserialize(attributes: Vec<String>) -> Self{
        let id = attributes[0].chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();
        let nombre= attributes[1].chars().collect::<String>().trim().to_string();
        let album = attributes[2].chars().collect::<String>().trim().to_string();
        let genero  = attributes[3].chars().collect::<String>().trim().to_string();
        let anio = attributes[4].chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();
        let duracion = attributes[5].chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();

        Song{
            id,
            nombre,
            album,
            genero,
            anio,
            duracion
        }
    }

    fn serialize(&self) -> String{
        format!("{:^10}{:^10}{:^10}{:^10}{:^10}{:^10}\n", 
        self.id, self.nombre,self.album,self.genero,
        self.anio, self.duracion
        )
    }
}

impl Song {
    pub fn new(id:u32) -> Song {
        println!("Ingrese el nombre de la cancion: ");
        let nombre = get_input();
        println!("Ingrese los album de la cancion: ");
        let album = get_input();
        println!("Ingrese el genero de la cancion: ");
        let genero = get_input();
        println!("Ingrese el anio de la cancion: ");
        let anio = get_input().parse::<u32>().unwrap();
        println!("Ingrese el duracion de la cancion: ");
        let duracion = get_input().parse::<u32>().unwrap();
        Song{
            id,
            nombre,
            album,
            genero,
            anio,
            duracion,
        }
    }
}
