use auxiliary::*;

use clap::{App, Arg};
use encoding::all::UTF_8;
use encoding::{EncoderTrap, Encoding};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod auxiliary;
mod node;

fn main() {
    let matches = App::new("Huffle")
        .version("0.1.0")
        .author("Void04 <xiayuxuan@live.com>")
        .about("用Rust语言实现的霍夫曼编码实例。")
        .arg(
            Arg::with_name("string")
                .short("s")
                .long("string")
                .value_name("source_string")
                .help("输入字符串")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("file")
                .help("输入文件")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("decode")
                .short("d")
                .long("decode")
                .value_name("decode")
                .help("将解码输入的文件/字符串")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("long")
                .value_name("output")
                .help("设置输出文件")
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("file") && matches.is_present("string") {
        panic!("不可同时输入字符串和文件！");
    }
    if !matches.is_present("file") && !matches.is_present("string") {
        panic!("未提供输入！");
    }

    let decode = matches.is_present("decode");
    let output = matches.is_present("output");

    let mut target = String::new();
    let mut result = String::new();

    let mut huff_code = String::new();

    if matches.is_present("string") {
        target = matches.value_of("string").unwrap().to_string();
        if decode {
            let target_arr: Vec<char> = target.chars().collect();
            let mut code_len = String::new();
            let mut code_len_len = 0;
            for each in target_arr {
                code_len_len += 1;
                if each == '/' {
                    break;
                }
                code_len.push(each);
            }
            let code_len: u32 = code_len.parse().unwrap();
            let huff_tree = gen_huff_tree_from_code(
                &target[code_len_len as usize..(code_len_len + code_len) as usize],
            );
            let huff_dict = gen_encoding_dict(huff_tree);
            result = huff_decode_str(&huff_dict, &target[(code_len_len + code_len) as usize..]);
        }
    } else {
        let input_path = Path::new(matches.value_of("file").unwrap());
        let input_display = input_path.display();
        let mut input_file = match File::open(&input_path) {
            Err(reason) => panic!("打开输入文件{}时出错：{}", input_display, reason),
            Ok(file) => file,
        };
        if decode {
            let mut input_vec = Vec::new();
            input_file
                .read_to_end(&mut input_vec)
                .expect("读取输入文件时出错");
            let parse_result = parse_bytes(input_vec);
            target = parse_result.1;
            huff_code = parse_result.0;
            let huff_tree = gen_huff_tree_from_code(&huff_code);
            let huff_dict = gen_encoding_dict(huff_tree);
            result = huff_decode_str(&huff_dict, &target);
        } else {
            input_file
                .read_to_string(&mut target)
                .expect("读取输入文件时出错");
        }
    }
    if !decode {
        let dict = gen_freq_dict(&target, None);
        let mut node_arr = gen_node_arr(dict);
        let huff_tree = gen_huff_tree_from_dict(&mut node_arr);
        let huff_dict = gen_encoding_dict(huff_tree.clone());
        let encoded_str = huff_encode_str(&huff_dict, &target);
        huff_code = gen_huff_tree_code(huff_tree);
        if !output {
            result = huff_code.len().to_string();
            result.push('/');
            result.push_str(&huff_code);
            result.push_str(&encoded_str);
            println!("编码结果为：\n{}", &result);
            println!("编码长度为：{}", result.len());
            let comp_rate = (64 as f64
                + 8 as f64 * UTF_8.encode(&huff_code, EncoderTrap::Strict).unwrap().len() as f64
                + 8 as f64 * UTF_8.encode(&"/", EncoderTrap::Strict).unwrap().len() as f64
                + encoded_str.len() as f64)
                / (8 as f64 * UTF_8.encode(&target, EncoderTrap::Strict).unwrap().len() as f64);
            println!("压缩率：{}", comp_rate);
        } else {
            result = encoded_str;
        }

        if output {
            let output_path = Path::new(matches.value_of("output").unwrap());
            let output_display = output_path.display();
            let mut output_file = match File::create(&output_path) {
                Err(reason) => panic!("创建输出文件{}时出错：{}", output_display, reason),
                Ok(file) => file,
            };
            match output_file.write_all(&gen_bytes(
                &huff_code,
                (8 - result.len() % 8) as u32,
                &result,
            )) {
                Err(reason) => panic!("写入输出文件{}时出错：{}", output_display, reason),
                Ok(_) => println!("写入完毕"),
            };
        }
    } else {
        if !output {
            println!("解码结果为：{}", result);
        } else {
            let output_path = Path::new(matches.value_of("output").unwrap());
            let output_display = output_path.display();
            let mut output_file = match File::create(&output_path) {
                Err(reason) => panic!("创建输出文件{}时出错：{}", output_display, reason),
                Ok(file) => file,
            };
            match output_file.write(result.as_bytes()) {
                Err(reason) => panic!("写入输出文件{}时出错：{}", output_display, reason),
                Ok(_) => println!("写入完毕"),
            };
        }
    }
}
