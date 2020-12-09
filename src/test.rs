#[cfg(test)]
mod tests {
    use crate::auxiliary::{
        binary_string_to_bytes, bytes_to_binary_string, gen_encoding_dict, gen_freq_dict,
        gen_huff_tree_code, gen_huff_tree_from_code, gen_huff_tree_from_dict, gen_node_arr,
        huff_decode_str, huff_encode_str,
    };

    use encoding::all::ASCII;
    use encoding::{EncoderTrap, Encoding};

    #[test]
    fn basic_comp() {
        let target = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string();
        let dict = gen_freq_dict(&target, None);
        let mut node_arr = gen_node_arr(dict);
        let huff_tree = gen_huff_tree_from_dict(&mut node_arr);
        let huff_code = gen_huff_tree_code(huff_tree.clone());
        let huff_code_bin: String = huff_code.bytes().map(|x| format!("{:b}", x)).collect();
        let huff_dict = gen_encoding_dict(huff_tree);
        let encoded = huff_encode_str(&huff_dict, &target);
        println!("Huffle编码结果：\n{}", &encoded);
        println!("总长度：{}", encoded.len());
        println!("编码文件头：\n{}", huff_code_bin);
        let decode_huff_tree = gen_huff_tree_from_code(&huff_code);
        let decode_dict = gen_encoding_dict(decode_huff_tree);
        let decoded = huff_decode_str(&decode_dict, &encoded);
        println!("Huffle解码结果为：{}", decoded);
        let ascii_encoded: String = ASCII
            .encode(&target, EncoderTrap::Strict)
            .unwrap()
            .into_iter()
            .map(|x| format!("{:b}", x))
            .collect();
        println!("Ascii编码结果：\n{}", ascii_encoded);
        println!("总长度：{}", ascii_encoded.len());
        let comp_rate = encoded.len() as f64 / ascii_encoded.len() as f64;
        println!("压缩率：{}", comp_rate);
    }

    #[test]
    fn string_to_bytes() {
        let original = format!("{:b}", 1145141919);
        println!("原二进制字符串为：{}", &original);
        let parsed_bytes = binary_string_to_bytes(original);
        println!("转换为bytes后为：{:?}", &parsed_bytes);
        let parsed_string = bytes_to_binary_string(parsed_bytes);
        println!("转换回二进制字符串为：{}", parsed_string);
    }
}
