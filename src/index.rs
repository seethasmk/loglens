use std::collections::HashMap;

//builds a search index that maps each word to the line numbers it appears in
pub fn build_index(logs: &Vec<String>) -> HashMap<String, Vec<usize>> {
    let mut index: HashMap<String, Vec<usize>> = HashMap:: new();
    for(i, line) in logs.iter().enumerate(){
        for word in line.split_whitespace(){
            index.entry(word.to_lowercase()).or_insert_with(Vec::new).push(i);
        }
    }
    index
}

//looks a keyword in the index and returns the matching line numbers
pub fn search(index: &HashMap<String, Vec<usize>>, keyword: &str) -> Vec<usize>{
    match index.get(&keyword.to_lowercase()){
        Some(list) => list.clone(),
        None => Vec::new(),
    }
}
