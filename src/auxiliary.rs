pub mod functons {
    use std::collections::HashMap;

    pub fn gen_freq_dict(s: String) -> HashMap<char, i32> {
        let mut dict: HashMap<char, i32> = HashMap::new();
        for ch in s.chars() {
            let count = dict.entry(ch).or_insert(0);
            *count += 1;
        }
        dict
    }
}
