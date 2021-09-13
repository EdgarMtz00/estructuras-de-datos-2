use std::io::{stdout, stdin};
use campos_dimension::file_storage::{FileStorage};

mod students;

fn main() {
    let filename = "estudiantes.txt";
    let mut file = FileStorage::<students::Student>::new(filename.to_string());
    let mut id = file.next_id();
    let mut option = String::new();
    while option != "6"{
        println!("1.- Agregar estudiante\n2.-Mostrar estudiantes\n3.-Buscar estudiante\n4.-Modificar estudiante\n5.-Eliminar estudiante\n6.-Salir\n");
    
        option = get_input();

        match &option[..] {
            "1" => {
                let student = students::Student::new(id);
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
                    println!("No es un identificador valido para el estudiante");
                    0
                });
                let result = file.search(num);
                match result {
                    Some(data) => {
                        println!("{}\n", data.to_string());
                    },
                    None => {
                        println!("No se encontro ese estudiante\n");
                    }
                }
            },
            "4" => {
                println!("Ingrese el numero del alumno a modificar\n");
                let num = get_input().parse::<u32>().unwrap_or_else(|_| {
                    println!("No es un identificador valido para el estudiante");
                    0
                });
                if num != 0{
                    let student = students::Student::new(num);
                    let _ = file.modify(num, student).unwrap_or_else(|_| {
                        println!("No se encontro el estudiante a modificar");
                        ()
                    });
                }
            },
            "5" => {
                println!("Ingrese el numero del estudiante a eliminar");
                let num = get_input().parse::<u32>().unwrap_or_else(|_| {
                    println!("No es un identificador valido para el estudiante");
                    0
                });
                if num != 0 {
                    match file.delete(num){
                        Ok(_) => println!("Estudiante eliminado"),
                        Err(_) => println!("No se encontro el estudiante a eliminar")
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
