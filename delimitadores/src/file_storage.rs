use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, Error, BufRead, SeekFrom, Seek};
use std::fmt;
use std::marker::PhantomData;

pub trait Deserializable{
    fn new (attributes: Vec<String>) -> Self;
}

pub struct FileStorage <T: fmt::Display> {
    _marker: PhantomData<T>,
    file_output: File,
    buffered_reader: BufReader<File>,
    cursor: u64,
}

impl <T: fmt::Display> FileStorage<T> {
    pub fn new(filename: String) -> Self {
        let output = match OpenOptions::new().write(true).open(&filename){
            Ok(file) => file,
            Err(e) => panic!("Can't edit file due to: {}", e),
        };

        let reader = match File::open(filename.to_string()){
            Ok(file) =>  BufReader::new(file),
            Err(e) => panic!("Can't open file due to: {}", e),
        };

        FileStorage::<T> {
            file_output: output,
            buffered_reader: reader,
            cursor: 0,
            _marker: PhantomData
        }
    }

    pub fn write(&mut self, record: &T) -> Result<(), Error> {
        &self.file_output.seek(SeekFrom::Start(self.cursor));
        write!(&self.file_output, "{}", record.to_string())?;
        self.cursor += record.to_string().len() as u64;
        Ok(())
    }

    pub fn read_line<'buf>(&mut self, buffer:&'buf mut String, restart: bool) -> Option<&'buf mut String>{
        buffer.clear();
        self.cursor = if restart {0} else {self.cursor};
        self.buffered_reader.seek(SeekFrom::Start(self.cursor));
        let line = self.buffered_reader.read_line(buffer).map(|u| if u == 0 {None} else {Some(buffer)}).transpose();
        self.cursor = self.buffered_reader.seek(SeekFrom::Current(0)).unwrap();
        if line.is_none() {
            return None;
        }
        match line.unwrap() {
            Ok(data) => {
                if data.is_empty(){
                    None
                }else{
                    Some(data)
                }
            },
            Err(_) => None,
        }
    }

    fn read_attribute(&self, line: &mut String ) -> Option<String>{
        let attr : String = line.chars().take_while(|&c| c != '|').collect();
        if attr.is_empty() {
            return None;
        }
        match line.char_indices().nth(attr.len() + 1){
            Some((pos, _)) => {
                line.drain(..pos);
            }
            None => {
                line.clear();
            }
        }
        Some(attr)
    }

    pub fn search(&mut self, first_element: impl PartialEq + fmt::Display) -> Option<T> where T: Deserializable{
        let mut buffer = String::new();
        let mut reset = true;
        while let Some(mut line) = self.read_line(&mut buffer, reset) {
            reset = false;
            let line_start = line.len();
            let attribute = self.read_attribute(&mut line).unwrap();
            if attribute == first_element.to_string() {
                self.cursor -= line_start as u64;
                let mut line = self.read_line(& mut buffer, false).unwrap();
                match self.deserialize(&mut line){
                    Ok(data) => {
                        return Some(data)
                    }
                    Err(e) => {
                        panic!("Data is corrupted");
                    }
                }
            }
        }
        None
    }

    pub fn deserialize(&self, line: &mut String) -> Result<T, Error> where T: Deserializable{
        let mut attributes = Vec::new();
        while let Some(attr) = self.read_attribute(line){
            attributes.push(attr);
        }
        Ok(T::new(attributes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug)]
    struct Record {
        id: u32,
        data: String,
    }

    impl fmt::Display for Record {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}|{}|\n", self.id, self.data)
        }
    }

    impl Deserializable for Record{
        fn new(attributes: Vec<String>) -> Self{
            Record{
                id: attributes[0].parse::<u32>().unwrap(),
                data: attributes[1].to_string(),
            }
        }
    }

    #[test]
    fn file_operations() -> Result<(), Error>{
        let filename = "test.txt";
        let mut file = FileStorage::<Record>::new(filename.to_string());
        let records = vec!{Record{id: 1, data: String::from("data")}, Record{id: 2, data: String::from("data 2")}};

        for record in &records {
            file.write(&record)?;
        }

        let mut buffer = String::new();

        let mut reset = true;
        let mut i = 0;
        while let Some(line) = file.read_line(&mut buffer, reset){
            reset = false;
            assert_eq!(file.deserialize(line).unwrap(), records[i]);
            i += 1;
        }

        Ok(())
    }

    #[test]
    fn search() -> Result<(), Error>{
        let filename = "test.txt";
        let mut file = FileStorage::<Record>::new(filename.to_string());
        let records = vec!{Record{id: 1, data: String::from("data")}, Record{id: 2, data: String::from("data 2")}};

        for record in &records {
            file.write(&record)?;
        }
        assert_eq!(file.search(2).unwrap(), records[1]);

        Ok(())
    }
}





