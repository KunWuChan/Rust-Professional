use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use serde_json;

type JsonInnerData = HashMap<String, HashMap<String, Vec<String>>>;

// 并查集：查找（路径压缩）
fn find(a: &str, parent: &mut HashMap<String, String>) -> String {
    if let Some(p) = parent.get(a) {
        if p != a {
            let p_clone = p.clone();
            let root = find(&p_clone, parent);
            parent.insert(a.to_string(), root.clone());
            return root;
        }
    }
    a.to_string()
}

// 并查集：合并（按秩优化）
fn merge(a: &str, b: &str, parent: &mut HashMap<String, String>, rank: &mut HashMap<String, i32>) {
    let pa = find(a, parent);
    let pb = find(b, parent);
    if pa != pb {
        let rank_a = rank.get(&pa).unwrap_or(&0);
        let rank_b = rank.get(&pb).unwrap_or(&0);
        if rank_a < rank_b {
            parent.insert(pa.clone(), pb.clone());
        } else if rank_a > rank_b {
            parent.insert(pb.clone(), pa.clone());
        } else {
            parent.insert(pa.clone(), pb.clone());
            rank.insert(pb.clone(), rank_b + 1);
        }
    }
}

// 计算连通分量数
fn dsu(data: &HashMap<String, Vec<String>>) -> i32 {
    let mut parent: HashMap<String, String> = HashMap::new();
    let mut rank: HashMap<String, i32> = HashMap::new();

    // 初始化节点
    for (key, value) in data {
        parent.entry(key.clone()).or_insert_with(|| key.clone());
        rank.entry(key.clone()).or_insert(0);
        for i in value {
            parent.entry(i.clone()).or_insert_with(|| i.clone());
            rank.entry(i.clone()).or_insert(0);
        }
    }

    // 合并连通分量
    for (key, value) in data {
        for i in value {
            if key != i { // 忽略自环
                merge(key, i, &mut parent, &mut rank);
            }
        }
    }

    // 统计根节点数
    let nodes: Vec<String> = parent.keys().cloned().collect();
    let mut roots = HashSet::new();
    for node in &nodes {
        roots.insert(find(node, &mut parent));
    }
    roots.len() as i32
}

pub fn count_provinces() -> String {
    let mut file = match File::open("district.json") {
        Ok(file) => file,
        Err(_) => return "".to_string(),
    };
    let mut json_str = String::new();
    if file.read_to_string(&mut json_str).is_err() {
        return "".to_string();
    }

    let raw_data: JsonInnerData = match serde_json::from_str(&json_str) {
        Ok(data) => data,
        Err(_) => return "".to_string(),
    };

    let mut data: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    for (batch, inner) in raw_data {
        let entry = data.entry(batch).or_insert_with(HashMap::new);
        for (key, mut value) in inner {
            value.sort();
            value.dedup();
            entry.entry(key).or_insert_with(Vec::new).extend(value);
        }
        for value in entry.values_mut() {
            value.sort();
            value.dedup();
        }
    }

    let mut result = Vec::new();
    for (batch, value) in data.iter() {
        let count = dsu(value);
        println!("Batch {}: {}", batch, count); // 调试输出
        result.push(count);
    }

    result.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",")
}
