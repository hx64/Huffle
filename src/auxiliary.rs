use crate::node;
use std::collections::HashMap;

pub fn gen_freq_dict(s: &str) -> HashMap<char, i32> {
    let mut dict: HashMap<char, i32> = HashMap::new();
    for ch in s.chars() {
        let count = dict.entry(ch).or_insert(0);
        *count += 1;
    }
    dict
}

pub fn gen_node_arr(dict: HashMap<char, i32>) -> Vec<node::Node> {
    let mut result = Vec::new();
    for (ch, weight) in dict.into_iter() {
        result.push(node::Node::from_char(weight, ch));
    }
    result
}

pub fn gen_huff_tree(arr: &mut Vec<node::Node>) -> node::Node {
    while arr.len() > 1 {
        let left_node = node::get_rarest(arr);
        let right_node = node::get_rarest(arr);
        let new_node =
            node::Node::from_children(Some(Box::new(left_node)), Some(Box::new(right_node)));
        arr.push(new_node);
    }
    arr.remove(0)
}

pub fn gen_encoding_dict(tree_top: node::Node) -> HashMap<char, String> {
    let mut result = HashMap::new();
    let empty_encoding_str = String::new();
    traverse_node_tree(empty_encoding_str, &mut result, tree_top);
    result
}

fn traverse_node_tree(curr: String, dict: &mut HashMap<char, String>, target: node::Node) {
    let mut tmp_encoding_str_left = curr.clone();
    if let Some(child) = target.left {
        tmp_encoding_str_left.push('0');
        traverse_node_tree(tmp_encoding_str_left, dict, *child);
    }
    let mut tmp_encoding_str_right = curr.clone();
    if let Some(child) = target.right {
        tmp_encoding_str_right.push('1');
        traverse_node_tree(tmp_encoding_str_right, dict, *child);
    }
    if let Some(value) = target.content {
        dict.entry(value).or_insert(curr);
    }
}

pub fn huff_encode_str(dict: &HashMap<char, String>, source: String) -> String {
    let mut result = String::new();
    for each in source.chars() {
        result.push_str(dict.get(&each).unwrap())
    }
    result
}

pub fn huff_decode_str(dict: &HashMap<char, String>, source: String) -> String {
    let mut result = String::new();
    let mut source_arr = source.chars().collect::<Vec<char>>();
    let mut rev_dict: HashMap<String, char> = HashMap::new();
    for (key, value) in dict.iter() {
        rev_dict.entry(value.to_owned()).or_insert(key.to_owned());
    }
    while !source_arr.is_empty() {
        let mut partial = String::new();
        while rev_dict.get(&partial).is_none() {
            partial.push(source_arr.remove(0));
        }
        result.push(rev_dict.get(&partial).unwrap().to_owned());
    }
    result
}
