/*! 
core barcode crate
*/

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::io;
use std::fs;

use regex::Regex;
use itertools::{Itertools};



/** 
Parse a pattern string into Hashmap.

- C: cell barcode
- L: linker
- U: UMI
- T: poly T
- N: placeholder

# Example
```rust
let pattern = "C8L16C8L16C8L1U12T18";
let dict = barcode::parse_pattern(pattern);
```
*/
pub fn parse_pattern(pattern: &str) -> HashMap<char, Vec<[usize;2]>> {
    let mut dict = HashMap::new();

    let re = Regex::new(r"([CLUNT])(\d+)").unwrap();

    let mut start = 0;
    for cap in re.captures_iter(pattern) {
        let symbol: char = cap[1].parse().unwrap();
        let length: usize = cap[2].parse().unwrap();
        let end = start + length;
        dict.entry(symbol).or_insert(Vec::new()).push([start, end]);
        start = end;
    }
    dict
}

/**
Find all sequences with at most n_mismatch compared to seq.
 */
pub fn findall_mismatch(seq: &str, n_mismatch: usize) -> HashSet<String> {
    let allowed_bases="ACGTN".chars();
    let mut result = HashSet::new();
    let seq_length = seq.len();

    for positions in (0..seq_length).combinations(n_mismatch) {
        let mut all_combinations: Vec<Vec<char>> = seq.
            chars().map(|c| vec![c]).collect();
        for position in positions {
            all_combinations[position] = allowed_bases.clone().collect();
        }
        let products = all_combinations.into_iter().
            map(|i| i.into_iter()).multi_cartesian_product();
        for product in products {
            result.insert(product.into_iter().collect());
        }
    }
    result
}

/**
Use findall_mismatch on a set of sequences.

Returns a Hashmap where
- key: mismatch sequence
- value: original sequence(e.g. barcode in whitelist)
 */
pub fn get_mismatch_dict(seq_list: &HashSet<String>, n_mismatch: usize) -> HashMap<String, String> {
    let mut mismatch_dict = HashMap::new();

    for seq in seq_list {
        for mismatch_seq in findall_mismatch(&seq, n_mismatch) {
            mismatch_dict.insert(mismatch_seq, seq.clone());
        }
    }
    mismatch_dict
}

#[test]
fn test_get_mismatch_dict() {
    let mut seq_list = HashSet::new();
    seq_list.insert("AACGTGAT".to_string());
    seq_list.insert("AAACATCG".to_string());
    let mismatch_dict = get_mismatch_dict(&seq_list, 1);
    let key = "AACGTGAA".to_string();
    let value = "AACGTGAT".to_string();
    assert_eq!(mismatch_dict.get(&key).unwrap(), &value);
}

/**
Read a file with one column into a HashSet.
 */
pub fn read_one_col<P: AsRef<Path>>(path: P) -> HashSet<String> {
    let mut set = HashSet::new();
    let content = fs::read_to_string(path).unwrap();
    for line in content.lines() {
        set.insert(line.trim().to_string());
    }
    set
}

/// Get str slice of pattern from sequence
pub fn get_pattern_seq<'a>(seq: &'a str, pattern_dict: &HashMap<char, Vec<[usize;2]>>, symbol: char) 
    -> Vec<&'a str> {
    let mut pattern_seq = Vec::new();
    for item in pattern_dict.get(&symbol).unwrap() {
        pattern_seq.push(&seq[item[0]..item[1]]);
    }
    pattern_seq
}


/**
Check if a sequence's pattern_seq is in mismatch_dict
*/ 
pub fn check_seq_mismatch(pattern_seq: Vec<&str>, mismatch_dict_list: &Vec<HashMap<String, String>>)
    -> Option<String> {
        for (index, sub_seq) in pattern_seq.iter().enumerate() {
            let mismatch_dict = &mismatch_dict_list[index];
            let sub_string = sub_seq.to_string();
            match mismatch_dict.get(&sub_string) {
                Some(_) => (),
                None => return None,
            }
        }
        let corrected_seq = pattern_seq.iter().map(|x| *x).collect();
        Some(corrected_seq)
    }

#[test]
fn test_check_seq_mismatch() {
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_pattern(){
        let pattern = "C8L16C8L16C8L1U12T18";
        let dict = parse_pattern(pattern);
        let pattern_c = dict.get(&'C').unwrap();
        let answer_c: Vec<[usize;2]> = vec![[0, 8], [24, 32], [48, 56]];
        assert_eq!(pattern_c, &answer_c);
    }

    #[test]
    fn test_findall_mismatch(){
        let mut answer = HashSet::new();
        let answer_array = [
            "TCG", "AAG", "ACC", "ATG", "ACT", "ACN", "GCG", 
            "ANG", "ACA", "ACG", "CCG", "AGG", "NCG",
        ];
        for s in answer_array {
            answer.insert(s.to_string());
        }

        let seq = "ACG".to_string();
        let value = findall_mismatch(&seq, 1);
        assert_eq!(value, answer);
    }

    #[test]
    fn test_read_one_col() {
        let path = "./Cargo.toml";
        let set = read_one_col(path);
        assert!(set.contains("[package]"));
    }
}
    