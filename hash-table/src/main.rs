use std::io::{stdout, stdin};

mod hash_table;

use hash_table::{HashTable, Entry};

fn main() {
    let mut option = String::new();
    let mut hash_table = HashTable::new();

    while option != "5"{
        println!("1.- Agregar/Modificar\n2.-Mostrar\n3.-Buscar\n4.-Eliminar\n5.-Salir\n");
    
        option = get_input();

        match &option[..] {
            "1" => {
                println!("Ingrese el termino: ");
                let termino = get_input();
                println!("Ingrese su definicion: ");
                let definicion = get_input();

                hash_table.append(Entry{termino, definicion});
            },
            "2" => {
                hash_table.show_all();
            },
            "3" => {
                println!("Ingrese el termino a buscar: ");
                let termino = get_input();
                let definicion = String::new();

                match hash_table.get(&Entry{termino, definicion}) {
                    Some(entry) => println!("{} => {}\n", entry.termino, entry.definicion),
                    None => println!("No se encontro el termino"),
                }
            },
            "4" => {
                println!("Ingrese el termino a eliminar: ");
                let termino = get_input();
                let definicion = String::new();

                hash_table.remove(&Entry{termino, definicion}); 
            },
            "5" => {
               println!("Saliendo...");
            }, 
            _ => {
                println!("Opcion Invalida");
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