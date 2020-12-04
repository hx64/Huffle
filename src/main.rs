use auxiliary::{
    gen_encoding_dict, gen_freq_dict, gen_huff_tree, gen_node_arr, huff_decode_str, huff_encode_str,
};

extern crate encoding;

mod auxiliary;
mod node;
fn main() {
    let target = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string();
    let dict = gen_freq_dict(&target);
    let mut node_arr = gen_node_arr(dict);
    let huff_tree = gen_huff_tree(&mut node_arr);
    let huff_dict = gen_encoding_dict(huff_tree);
    let result = huff_encode_str(&huff_dict, target);
    println!("{}", &result);
    println!("length:{}", result.len());
    let decoded = huff_decode_str(&huff_dict, result);
    println!("{}", decoded);
}
