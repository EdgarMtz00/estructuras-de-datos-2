use std::io::{stdout, stdin};
use campos_dimension::file_storage::{FileStorage};

mod song;

fn main() {
    let filename = "canciones.txt";
    let mut file = FileStorage::<song::Song>::new(filename.to_string());
    let mut id = file.next_id();
    let mut option = String::new();
    while option != "6"{
        println!("1.- Agregar cancion\n2.-Mostrar canciones\n3.-Buscar cancion\n4.-Modificar cancion\n5.-Eliminar cancion\n6.-Salir\n");
    
        option = get_input();

        match &option[..] {
            "1" => {
                let student = song::Song::new(id);
                id += 1;
                file.write(&student);
            },
            "2" => {
                let mut buffer = String::new();
                let mut reset = true;
                while let Some(mut line) = file.read_line(&mut buffer, reset) {
                    reset = false;
                    if line.chars().filter(|&c| c != ' ').collect::<String>().len() > 5 {
                        println!("{}", file.deserialize(&mut line).unwrap());
                    }
                }
                println!("\n");
            },
            "3" => {
                println!("Ingrese el numero del alumno a buscar\n");
                let num = get_input().parse::<u32>().unwrap_or_else(|_| {
                    println!("No es un identificador valido para la cancion");
                    0
                });
                let result = file.search(num);
                match result {
                    Some(data) => {
                        println!("{}\n", data.to_string());
                    },
                    None => {
                        println!("No se encontro esa cancion\n");
                    }
                }
            },
            "4" => {
                println!("Ingrese el numero de la cancion a modificar\n");
                let num = get_input().parse::<u32>().unwrap_or_else(|_| {
                    println!("No es un identificador valido para la cancion");
                    0
                });
                if num != 0{
                    let student = song::Song::new(num);
                    let _ = file.modify(num, student).unwrap_or_else(|_| {
                        println!("No se encontro la cancion a modificar");
                        ()
                    });
                }
            },
            "5" => {
                println!("Ingrese el numero de la cancion a eliminar");
                let num = get_input().parse::<u32>().unwrap_or_else(|_| {
                    println!("No es un identificador valido para la cancion");
                    0
                });
                if num != 0 {
                    match file.delete(num){
                        Ok(_) => println!("Cancion eliminado"),
                        Err(_) => println!("No se encontro la cancion a eliminar")
                    }
                }
            },
            "6" => {
                file.save();
                println!("Saliendo...");
            },
            _ => {

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
