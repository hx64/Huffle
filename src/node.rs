use core::panic;    // 导入panic错误处理模块

// 定义节点结构体
#[derive(Debug, Clone)]
pub struct Node {
    pub weight: i32,
    pub content: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    // 定义从子节点生成节点的构造函数
    pub fn from_children(left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
        let mut left_value = 0;
        let mut right_value = 0;
        if let Some(inner) = &left {
            left_value = inner.weight;
        }
        if let Some(inner) = &right {
            right_value = inner.weight;
        }

        Node {
            weight: left_value + right_value,
            content: None,
            left: left,
            right: right,
        }
    }

    // 定义从字符值生成节点的构造函数
    pub fn from_char(weight: i32, content: char) -> Node {
        Node {
            weight,
            content: Some(content),
            left: None,
            right: None,
        }
    }
}

// 获取一组节点中权重最小的节点的函数
pub fn get_rarest(arr: &mut Vec<Node>) -> Node {
    if arr.is_empty() {
        panic!("尝试对空数组取最小值");
    }
    let mut result_index = 0;
    for i in 0..arr.len() {
        if arr[i].weight < arr[result_index].weight {
            result_index = i;
        }
    }
    arr.swap_remove(result_index)
}
