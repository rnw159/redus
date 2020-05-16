
// use std::convert::TryInto;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

// const DEFAULT_HASH_TABLE_SIZE: usize = 1024*1024;
const DEFAULT_HASH_TABLE_SIZE: usize = 1024;

#[derive(Debug)]
struct HashNode {
    key: String,
    value: String,
    next: Option<Box<HashNode>>,
}

#[derive(Debug)]
pub struct HashTable {
    table: Vec<Option<HashNode>>,
}

impl HashTable {
    pub fn new() -> HashTable {
        let mut hash_table = HashTable{
            table: Vec::with_capacity(DEFAULT_HASH_TABLE_SIZE),
        };
        for _ in 0..DEFAULT_HASH_TABLE_SIZE {
            hash_table.table.push(None)
        }
        return hash_table
    }

    fn hash(&mut self, key: String) -> usize {
        let mut hasher = DefaultHasher::new();

        for c in key.chars(){
            let i = c as u32;
            hasher.write_u32(i);
        }
        let default_size = DEFAULT_HASH_TABLE_SIZE as u64;
        let index = hasher.finish() % default_size;
        let index = index as usize;
        return index
    }

    pub fn set(&mut self, key: String, value: String){
        let index = self.hash(key.clone());
        let hash_node = HashNode{
            key: key.clone(),
            value: value.clone(),
            next: None,
        };
        self.table[index] = Some(hash_node);
        
    }

    pub fn get(&mut self, key: String) -> Option<String>{
        let index = self.hash(key.clone());
        let result = &self.table[index];
        match result {
            Some(node) => return Some(node.value.clone()),
            None => return None
        }
    }

    pub fn delete(&mut self, key: String) -> Option<String>{
        let index = self.hash(key.clone());
        let result = self.get(key);
        match result {
            Some(value) => {
                self.table[index] = None;
                return Some(value.clone())
            },
            None => return None
        }
    }
}
