#[derive(Clone)]
pub struct Entry {
    pub termino: String,
    pub definicion: String,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.termino == other.termino
    }

    fn ne(&self, other: &Self) -> bool {
        self.termino != other.termino
    }
}

pub struct HashTable {
    pub array: [Vec<Entry>; 30],
} 

impl HashTable {
    pub fn new() -> HashTable {
        let array = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),
        Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),
        Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        HashTable {
            array
        }
    }

    fn hash(&self, value: &String) -> usize{
        let mut result: usize = 0;
        
        for (i, c) in value.chars().enumerate() {
            result += i * c as usize;
        }

        result % 30
    }

    pub fn append(&mut self, value: Entry) {
        let position = self.hash(&value.termino);
        if self.array[position].contains(&value) {
            self.remove(&value);
        }
        self.array[position].push(value);
    }

    fn find(&self, value: &Entry) -> Option<usize> {
        let position = self.hash(&value.termino);
        let mut i = 0;

        for entry in &self.array[position] {
            if entry == value {
                return Some(i); 
            }
            i += 1;
        }
        None
    }

    pub fn get(&self, value: &Entry) -> Option<Entry> {
        let position = self.hash(&value.termino);
        match self.find(value) {
            Some(i) => Some(self.array[position][i].clone()),
            None => None
        }
    }

    pub fn remove(&mut self, value: &Entry) {
        let position = self.hash(&value.termino);
        if let Some(i) = self.find(value) {
            self.array[position].remove(i);
        }
    }

    pub fn show_all(&self) {
        for list in &self.array {
            for entry in list{
                println!("{} => {}\n", entry.termino, entry.definicion);
            }
        }
    }
}