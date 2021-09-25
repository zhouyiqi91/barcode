/*! 
core barcode crate
*/

use std::collections::{HashMap, HashSet};

use regex::Regex;
use itertools::{Itertools};



/** 
Parse a pattern string into Hashmap 
- C: cell barcode
- L: linker
- U: UMI
- T: poly T
- N: placeholder

# Example
```rust
let pattern = "C8L16C8";
let dict = barcode::parse_pattern(pattern);
let pattern_c = dict.get(&'C').unwrap();
let answer_c: Vec<[u8;2]> = vec![[0, 8], [32, 40]];
assert_eq!(pattern_c, &answer_c);
```
*/
pub fn parse_pattern(pattern: &str) -> HashMap<char, Vec<[u8;2]>> {
    let mut dict = HashMap::new();

    let re = Regex::new(r"([CLUNT])(\d+)").unwrap();

    let mut start = 0;
    for cap in re.captures_iter(pattern) {
        let symbol: char = cap[1].parse().unwrap();
        let length: u8 = cap[2].parse().unwrap();
        let end = start + length;
        dict.entry(symbol).or_insert(Vec::new()).push([start, end]);
        start += end;
    }
    dict
}

/**
Returns all sequences with at most n_mismatch compared to seq.
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
    return result;
}


mod tests {
    use super::*;

    #[test]
    fn test_parse_pattern(){
        let pattern = "C8L16C8";
        let dict = parse_pattern(pattern);
        let pattern_c = dict.get(&'C').unwrap();
        let answer_c: Vec<[u8;2]> = vec![[0, 8], [32, 40]];
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
}
    