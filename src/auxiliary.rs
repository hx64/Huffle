use crate::node;

use std::collections::HashMap;  // 导入哈希表模块便于构造字符频率字典

use encoding::all::UTF_8;   // 导入encoding模块便于编码文件头
use encoding::{DecoderTrap, EncoderTrap, Encoding};

// 从字符串生成字符频率字典的函数
pub fn gen_freq_dict(s: &str, original_dict: Option<HashMap<char, i32>>) -> HashMap<char, i32> {
    let mut dict: HashMap<char, i32> = original_dict.unwrap_or(HashMap::new());
    for ch in s.chars() {
        let count = dict.entry(ch).or_insert(0);
        *count += 1;
    }
    dict
}

// 从字符频率字典生成节点数组的函数
pub fn gen_node_arr(dict: HashMap<char, i32>) -> Vec<node::Node> {
    let mut result = Vec::new();
    for (ch, weight) in dict.into_iter() {
        result.push(node::Node::from_char(weight, ch));
    }
    result
}

// 从节点组生成霍夫曼树并返回根节点的函数
pub fn gen_huff_tree_from_dict(arr: &mut Vec<node::Node>) -> node::Node {
    while arr.len() > 1 {
        let left_node = node::get_rarest(arr);
        let right_node = node::get_rarest(arr);
        let new_node =
            node::Node::from_children(Some(Box::new(left_node)), Some(Box::new(right_node)));
        arr.push(new_node);
    }
    arr.remove(0)
}

// 根据霍夫曼树（根节点）生成编码字典的函数
pub fn gen_encoding_dict(tree_top: node::Node) -> HashMap<char, String> {
    let mut result = HashMap::new();
    let empty_encoding_str = String::new();
    traverse_node_tree_for_dict(empty_encoding_str, &mut result, tree_top);
    result
}

// 函数gen_encoding_dict中使用的递归遍历函数
fn traverse_node_tree_for_dict(curr: String, dict: &mut HashMap<char, String>, target: node::Node) {
    let mut tmp_encoding_str_left = curr.clone();
    if let Some(child) = target.left {
        tmp_encoding_str_left.push('0');
        traverse_node_tree_for_dict(tmp_encoding_str_left, dict, *child);
    }
    let mut tmp_encoding_str_right = curr.clone();
    if let Some(child) = target.right {
        tmp_encoding_str_right.push('1');
        traverse_node_tree_for_dict(tmp_encoding_str_right, dict, *child);
    }
    if let Some(value) = target.content {
        dict.entry(value).or_insert(curr);
    }
}

// 通过编码字典编码字符串的函数
pub fn huff_encode_str(dict: &HashMap<char, String>, source: &str) -> String {
    let mut result = String::new();
    for each in source.chars() {
        result.push_str(dict.get(&each).unwrap())
    }
    result
}

// 通过编码字典解码字符串的函数
pub fn huff_decode_str(dict: &HashMap<char, String>, source: &str) -> String {
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

// 从霍夫曼树根节点生成文件头字符串的函数
pub fn gen_huff_tree_code(tree_top: node::Node) -> String {
    let mut result = String::new();
    traverse_node_tree_for_code(&mut result, tree_top);
    result
}

// 函数gen_huff_tree_code中使用的递归遍历函数
fn traverse_node_tree_for_code(curr: &mut String, target: node::Node) {
    if target.content.is_some() {
        curr.push('0');
        curr.push(target.content.unwrap());
    } else {
        traverse_node_tree_for_code(curr, *target.left.unwrap());
        traverse_node_tree_for_code(curr, *target.right.unwrap());
        curr.push('1');
    }
}

// 从文件头字符串生成霍夫曼树的函数
pub fn gen_huff_tree_from_code(code: &str) -> node::Node {
    let mut code_arr: Vec<char> = code.chars().collect();
    let mut node_arr = Vec::new();
    while !code_arr.is_empty() {
        match code_arr.remove(0) {
            '0' => {
                let new_node = node::Node::from_char(0, code_arr.remove(0));
                node_arr.push(new_node);
            }
            '1' => {
                let right_node = node_arr.pop().unwrap();
                let left_node = node_arr.pop().unwrap();
                let new_node = node::Node::from_children(
                    Some(Box::new(left_node)),
                    Some(Box::new(right_node)),
                );
                node_arr.push(new_node);
            }
            _ => panic!("编码头出现错误！"),
        }
    }
    node_arr.remove(0)
}

// 二进制字符串转换为字节数组的函数
fn binary_string_to_bytes(original: &str) -> Vec<u8> {
    let mut char_arr: Vec<char> = original.chars().collect();
    let mut result: Vec<u8> = Vec::new();
    while !char_arr.is_empty() {
        while char_arr.len() < 8 {
            char_arr.push('0');
        }
        let mut new_u8: u8 = 0;
        for i in 0..8 {
            let exponent = ((7 - i) as f32).exp2() as u8;
            new_u8 += (char_arr.remove(0).to_digit(10).unwrap() as u8) * exponent;
        }
        result.push(new_u8);
    }
    result
}

// 字节数组转换为二进制字符串的函数
fn bytes_to_binary_string(bytes: &[u8], cutoff: u32) -> String {
    let mut result = String::new();
    for each in bytes {
        result.push_str(&format!("{:08b}", each));
    }
    for _ in 0..cutoff as usize {
        result.pop();
    }
    result
}

// 生成写入文件的字符数组的函数
pub fn gen_bytes(huff_code: &str, cutoff: u32, code: &str) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let huff_code_encoded = UTF_8.encode(huff_code, EncoderTrap::Strict).unwrap();
    result.extend_from_slice(&(huff_code_encoded.len() as u32).to_be_bytes());
    result.extend_from_slice(&cutoff.to_be_bytes());
    result.extend_from_slice(&huff_code_encoded);
    result.extend_from_slice(&binary_string_to_bytes(code));
    result
}

// 从文件中读取的字节数组提取文件头字符串和编码字符串的函数
pub fn parse_bytes(bytes: Vec<u8>) -> (String, String) {
    let bytes_arr: &[u8] = &bytes;
    let mut huff_code_len_arr: [u8; 4] = Default::default();
    huff_code_len_arr.copy_from_slice(&bytes_arr[..4]);
    let huff_code_len = u32::from_be_bytes(huff_code_len_arr);
    let mut cutoff_arr: [u8; 4] = Default::default();
    cutoff_arr.copy_from_slice(&bytes_arr[4..8]);
    let cutoff = u32::from_be_bytes(cutoff_arr);
    let huff_code = UTF_8
        .decode(&bytes[8..(8 + huff_code_len as usize)], DecoderTrap::Strict)
        .unwrap();
    let encoded_str = bytes_to_binary_string(&bytes[(8 + huff_code_len as usize)..], cutoff);
    (huff_code, encoded_str)
}
