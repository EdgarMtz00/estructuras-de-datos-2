use std::io::{stdout, stdin};
use delimitadores::file_storage::{FileStorage};

mod calculator;

fn main() {
    let filename = "operaciones.txt";
    let mut file = FileStorage::<calculator::Operation>::new(filename.to_string());
    let mut id = file.next_id();
    let mut option = String::new();
    while option != "6"{
        println!("1.- Agregar operacion\n2.-Mostrar operaciones\n3.-Buscar operacion\n4.-Modificar operacion\n5.-Eliminar operacion\n6.-Salir\n");
    
        option = get_input();

        match &option[..] {
            "1" => {
                println!("Ingrese una nueva operacion (sin espacios)\n");
                let input = get_input();
                let operation = calculator::Operation::new(input, id);
                id += 1;
                match operation {
                    Ok(data) => {
                        file.write(&data);
                    },
                    Err(_) => {
                        println!("No es una operacion valida\n");
                    }
                }
            },
            "2" => {
                let mut buffer = String::new();
                let mut reset = true;
                while let Some(line) = file.read_line(&mut buffer, reset) {
                    reset = false;
                    println!("{}", line);
                }
                println!("\n");
            },
            "3" => {
                println!("Ingrese el numero de la operacion a buscar\n");
                let num = get_input().parse::<u32>().unwrap_or_else(|_| {
                    println!("No es un identificador valido para la operacion");
                    0
                });
                let result = file.search(num);
                match result {
                    Some(data) => {
                        println!("{}\n", data.to_string());
                    },
                    None => {
                        println!("No se encontro esa operacion\n");
                    }
                }
            },
            "4" => {
                println!("Ingrese el numero de la operacion a modificar\n");
                let num = get_input().parse::<u32>().unwrap_or_else(|_| {
                    println!("No es un identificador valido para la operacion");
                    0
                });
                if num != 0{
                    println!("Ingrese la nueva operacion");
                    let input = get_input();
                    let operation = calculator::Operation::new(input, num);
                    match operation {
                        Ok(operation) =>  {
                            let _ = file.modify(num, operation).unwrap_or_else(|_| {
                                println!("No se encontro la operacion a modificar");
                                ()
                            });
                        },
                        Err(e) => {
                            println!("No es una operacion valida");
                        }
                    }
                }
            },
            "5" => {
                println!("Ingrese el numero de la operacion a eliminar");
                let num = get_input().parse::<u32>().unwrap_or_else(|_| {
                    println!("No es un identificador valido para la operacion");
                    0
                });
                if num != 0 {
                    match file.delete(num){
                        Ok(_) => println!("Operacion eliminada"),
                        Err(_) => println!("No se encontro la operacion a eliminar")
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
