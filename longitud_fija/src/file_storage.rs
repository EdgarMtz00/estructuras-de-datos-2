use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, Error, BufRead, SeekFrom, Seek, ErrorKind};
use std::fmt;
use std::marker::PhantomData;

pub trait Deserializable{
    fn deserialize(attributes: Vec<String>) -> Self;
    fn serialize(&self) -> String;
}

pub struct FileStorage <T: Deserializable> {
    _marker: PhantomData<T>,
    file_output: File,
    buffered_reader: BufReader<File>,
    cursor: u64,
    insert_pos: Vec<u64>,
}

impl <T: Deserializable> FileStorage<T> {
    pub fn new(filename: String) -> Self {
        let mut output = match OpenOptions::new().write(true).open(&filename){
            Ok(file) => file,
            Err(e) if e.kind() == ErrorKind::NotFound => File::create(&filename).unwrap(),
            Err(e) => panic!("Can't edit file due to: {} ({:?})", e, e.kind()),
        };

        let mut reader = match File::open(filename.to_string()){
            Ok(file) =>  BufReader::new(file),
            Err(e) => panic!("Can't open file due to: {}", e),
        };

        let mut start_pos = 0;
        let mut buf = String::new();
        let mut cursor = 0;

        reader.read_line(&mut buf);
        if !buf.is_empty(){
            let line = buf.chars().take_while(|&c| c != '\n').collect::<String>();
            cursor = line.len() + 1;
            let line = buf.chars().take_while(|&c| c != ' ').collect::<String>();
            start_pos = line.parse::<u64>().unwrap();
        }else{
            write!(&mut output, "18               \n");
            start_pos = 18;
            cursor = 18;
        }

        FileStorage::<T> {
            file_output: output,
            buffered_reader: reader,
            cursor: cursor as u64,
            _marker: PhantomData,
            insert_pos: vec!(start_pos),
        }
    }

    pub fn write(&mut self, record: &T) -> Result<(), Error> {
        if self.insert_pos.len() == 1{
            &self.file_output.seek(SeekFrom::Start(self.insert_pos[0]));
            self.insert_pos[0] += record.serialize().len() as u64;
        }else{
            &self.file_output.seek(SeekFrom::Start(self.insert_pos.pop().unwrap()));
        }
        write!(&self.file_output, "{}", record.serialize())?;
        Ok(())
    }

    fn write_char(&mut self, data: char) -> Result<(), Error> {
        &self.file_output.seek(SeekFrom::Start(self.cursor));
        write!(&self.file_output, "{}", data)?;
        self.cursor += 1;
        Ok(())
    }

    pub fn read_line<'buf>(&mut self, buffer:&'buf mut String, restart: bool) -> Option<&'buf mut String>{
        buffer.clear();
        if restart {
            self.buffered_reader.seek(SeekFrom::Start(0));
            self.buffered_reader.read_line(buffer);
            buffer.clear();
        }else{
            self.buffered_reader.seek(SeekFrom::Start(self.cursor));
        }
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
        let size = 9;
        let attr : String = line.chars().take(size).collect();
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

    fn search_line<'buf>(&mut self, id:impl PartialEq + fmt::Display, buffer:&'buf mut String) -> Option<&'buf mut String>{
        let mut reset = true;
        
        while let Some(mut line) = self.read_line(buffer, reset) {
            reset = false;
            let line_start = line.len();
            let attribute = self.read_attribute(&mut line).unwrap().chars().filter(|&c| c.is_ascii_digit()).collect::<String>();
            if attribute == id.to_string() {
                self.cursor -= line_start as u64;
                return self.read_line(buffer, false)
            }
        }
        None
    }

    pub fn search(&mut self, id: impl PartialEq + fmt::Display) -> Option<T> where T: Deserializable{
        let mut buffer = String::new();
        match self.search_line(id, &mut buffer){
            Some(mut line) => {
                match self.deserialize(&mut line){
                    Ok(data) => {
                        return Some(data)
                    }
                    Err(_) => {
                        panic!("Data is corrupted");
                    }
                }
            },
            None => None
        }
    }

    pub fn delete(&mut self, id: impl PartialEq + fmt::Display) -> Result<(), Error>{
        let mut buffer = String::new();
        match self.search_line(id, &mut buffer){
            Some(line) => {
                self.cursor = self.cursor - line.len() as u64;
                self.insert_pos.push(self.cursor);
                for _ in line.chars(){
                    self.write_char(' ')?;
                }
                self.cursor -= 1;
                self.write_char('\n')?;
                Ok(())
            },
            None => Err(Error::new(ErrorKind::NotFound, "Not Found"))
        }
    }

    pub fn modify(&mut self, id: impl PartialEq + fmt::Display, new_record: T) -> Result<(), Error>{
        self.delete(id)?;
        self.write(&new_record)?;
        Ok(())
    }

    pub fn deserialize(&self, line: &mut String) -> Result<T, Error>{
        let mut attributes = Vec::new();
        while let Some(attr) = self.read_attribute(line){
            attributes.push(attr);
        }
        Ok(T::deserialize(attributes))
    }

    pub fn save(&mut self){
        let mut buffer = String::new();
        self.cursor = 0;
        let line = self.read_line(&mut buffer, false).unwrap();
        let line = line.chars().take_while(|&c| c != ' ').collect::<String>();

        let last_pos = self.insert_pos[0].to_string();

        if line.len() > last_pos.len(){
            self.cursor = 0;
            for _ in 0..line.len(){
                self.write_char(' ');
            }
        }

        self.cursor = 0;
        for digit in last_pos.chars() {
            self.write_char(digit);
        }
    }

    pub fn next_id(&mut self,) -> u32{
        let mut buffer = String::new();
        let mut reset = true;
        let mut id = 0;
        while let Some(mut line) = self.read_line(&mut buffer, reset){
            reset = false;
            let next = self.read_attribute(&mut line);
            if next.is_some() {
                let next = next.unwrap().chars().filter(|&c| c.is_ascii_digit()).collect::<String>();
                if !next.is_empty(){
                    let next = next.parse::<u32>().unwrap();
                    if next > id {
                        id = next;
                    }
                }
            }
        }
        id + 1
    }

    pub fn delete_all(&mut self) -> Result<(), Error>{
        let mut buffer = String::new();
        let mut reset = true;
        while let Some(mut line) = self.read_line(&mut buffer, reset){
            reset = false;
            let next = self.read_attribute(&mut line);
            if next.is_some() {
                let next = next.unwrap();
                if !next.chars().all(|c| c == ' ' || c == '\n') {
                    let next = next.parse::<u32>().unwrap();
                    self.delete(next)?;
                }
            }
        }
        self.insert_pos[0] = 18;
        Ok(())
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
        fn deserialize(attributes: Vec<String>) -> Self{
            Record{
                id: attributes[0].parse::<u32>().unwrap(),
                data: attributes[1].to_string(),
            }
        }
        fn serialize(&self) -> String{
            format!("{}|{}|\n", self.id, self.data)
        }
    }

    #[test]
    fn file_operations() -> Result<(), Error>{
        let filename = "test.txt";
        let mut file = FileStorage::<Record>::new(filename.to_string());
        let records = vec!{Record{id: 1, data: String::from("old data")}, Record{id: 2, data: String::from("data 2")}};

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

        file.modify(1, Record{id:1, data:String::from("new data")})?;
        assert_eq!(file.search(1).unwrap(),Record{id:1, data:String::from("new data")});
        assert_eq!(3, file.next_id());

        file.delete_all()?;
        file.save();
        Ok(())
    }

    #[test]
    fn search_delete_and_search_again() -> Result<(), Error>{
        let filename = "test.txt";
        let mut file = FileStorage::<Record>::new(filename.to_string());
        let records = vec!{Record{id: 1, data: String::from("data")}, Record{id: 2, data: String::from("data 2")}};

        for record in &records {
            file.write(&record)?;
        }
        assert_eq!(file.search(1).unwrap(), records[0]);

        file.delete(1)?;

        assert_ne!(file.search(1).unwrap_or(Record{id:0, data:String::from("")}), records[0]);
        assert_eq!(None, file.search(420));

        file.delete_all()?;
        file.save();
        Ok(())
    }
}