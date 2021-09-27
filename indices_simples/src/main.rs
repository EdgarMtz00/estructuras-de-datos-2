extern crate file_storage;

mod keys;
mod persons;

use file_storage::file_storage::FileStorage;
use keys::Key;
use persons::Person;
use std::io::{stdout, stdin};

fn main() {
    let data_file_name = "data.txt";
    let keys_file_name = "keys.txt";

    let mut data_file = FileStorage::<Person>::new(data_file_name.to_string());
    let mut keys_file = FileStorage::<Key>::new(keys_file_name.to_string());
    
    let mut id = keys_file.next_id();

    let mut option = String::new();

    while option != "6" {
        println!("1.- Agregar\n2.- Mostrar\n3.- Buscar\n4.- Modificar\n5.- Eliminar\n6.- Salir");
        option = get_input();
        match &option[..] {
            "1" => {
                let key = Key::new(id);
                let person = Person::new(id);
                id += 1;
                keys_file.write(&key);
                data_file.write(&person);
            },
            "2" => {
                let mut key_buffer = String::new();
                let mut data_buffer = String::new();
                let mut reset = true;
                while let Some(key) = keys_file.read_line(&mut key_buffer, reset) {
                    let data = data_file.read_line(&mut data_buffer, reset).unwrap();
                    reset = false;
                    println!("{}", key);
                    println!("{}", data)
                }
                println!("\n");
            },
            "3" => {
                println!("Ingrese la clave del registro a buscar\n");
                let num = get_input();
                let result = keys_file.search_by_key(&num, 2);
                match result {
                    Some(data) => {
                        println!("{}\n", data.to_string());
                        println!("{}\n", data_file.search(data.id).unwrap());
                    },
                    None => {
                        println!("No se encontro la clave\n");
                    }
                }
            },
            "4" => {
                println!("Ingrese la clave del registro a modificar\n");
                let num = get_input();
                let result = keys_file.search_by_key(&num, 2);
                match result {
                    Some(data) => {
                        let person = Person::new(data.id);
                        data_file.modify(data.id, person);
                    },
                    None => {
                        println!("No se encontro esa clave\n");
                    }
                }
            },
            "5" => {
                println!("Ingrese la clave del registro a eliminar\n");
                let num = get_input();
                let result = keys_file.search_by_key(&num, 2);
                match result {
                    Some(data) => {
                        data_file.delete(data.id);
                        keys_file.delete(data.id);
                    },
                    None => {
                        println!("No se encontro esa clave\n");
                    }
                }
            },
            "6" => {
                keys_file.save();
                data_file.save();
                println!("Saliendo...");
            }
            _ => {
                println!("Opcion no valida");
            }
        }
    }

}

fn get_input() -> String{
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }
    input
}
