use std::io::{Error, ErrorKind};
use delimitadores::file_storage::Deserializable;
use std::fmt;

pub struct Operation {
    pub id:u32, 
    pub first_coefficient: u32,
    pub second_coefficient: u32,
    pub operation: OperationType,
    pub result: u32
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:^5}|{:^8}|{}|{:^8}|{:^8}\n", self.id, self.first_coefficient, self.operation, self.second_coefficient, self.result)
    }
}

impl Deserializable for Operation{
    fn deserialize(attributes: Vec<String>) -> Self{
        let id = attributes[0].chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();
        let first_coefficient= attributes[1].chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();
        let operation = OperationType::from(&attributes[2]);
        let second_coefficient  = attributes[3].chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();
        let result = attributes[4].chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();

        Operation{
            id,
            first_coefficient,
            operation,
            second_coefficient,
            result,
        }
    }

    fn serialize(&self) -> String{
        format!("{:^5}|{:^8}|{}|{:^8}|{:^8}\n", self.id, self.first_coefficient, self.operation, self.second_coefficient, self.result)
    }
}

#[derive(PartialEq, Debug)]
pub enum OperationType {
    Sum,
    Rest, 
    Mult,
    Div,
    Invalid
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationType::Sum => write!(f, "+"),
            OperationType::Rest => write!(f, "-"),
            OperationType::Mult => write!(f, "*"),
            OperationType::Div => write!(f, "/"),
            OperationType::Invalid => panic!("Invalid operator"),
        }
    }
}

impl From<&String> for OperationType{
    fn from(operation: &String) -> Self {
        if operation == &String::from("+") {
            Self::Sum
        } else if operation == &String::from("-") {
            Self::Rest
        } else if operation == &String::from("*") {
            Self::Mult
        } else if operation == &String::from("/") {
            Self::Div
        } else {
            Self::Invalid
        }
    }
}

impl Operation {
    pub fn new(input: String, id:u32) -> Result<Operation, Error> {
        let first:String = input.chars().take_while(|&c| c != '+' && c != '-' && c != '*' && c != '/' && c != ' ').collect();
        let operation_char = input.chars().nth(first.len()).unwrap();
        let second = input.chars().skip(first.len() + 1).collect::<String>();

        let first_coefficient = first.parse::<u32>().unwrap();
        let second_coefficient = second.parse::<u32>().unwrap();

        let result:u32;
        let operation:OperationType;

        match operation_char {
            '+' => {
                result = first_coefficient + second_coefficient;
                operation = OperationType::Sum;
            },
            '-' => {
                result = first_coefficient - second_coefficient;
                operation = OperationType::Rest;
            },
            '*' => {
                result = first_coefficient * second_coefficient;
                operation = OperationType::Mult;
            },
            '/' => {
                result = first_coefficient / second_coefficient;
                operation = OperationType::Div;
            },
            _ =>{
                return Err(Error::new(ErrorKind::InvalidInput, "Not  valid operation symbol"));
            }
        };
        Ok(Operation{
            id,
            first_coefficient,
            second_coefficient,
            operation,
            result,
        })
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn create_operation() -> Result<(), Error> {
        let operation = Operation::new(String::from("2+5"), 1)?;
        assert_eq!(OperationType::Sum, operation.operation);
        assert_eq!(7, operation.result);
        Ok(())
    }

    #[test]
    pub fn print_operation() -> Result<(), Error> {
        let operation = Operation::new(String::from("2+5"), 1)?;
        assert_eq!(OperationType::Sum, operation.operation);
        assert_eq!("  1  |   2    |+|   5    |   7    \n", operation.to_string());
        Ok(())
    }
}