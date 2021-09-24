/*! 
Tools for handling barcode and UMI.
*/


use regex::Regex;
use std::collections::HashMap;

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
let dict = parse_pattern(pattern);
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
}
    