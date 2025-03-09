use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use serde_json;

type DistrictData = HashMap<String, HashMap<String, Vec<String>>>;

const DISTRICT_FILE: &str = "district.json";

#[derive(Debug)]
struct UnionFind<'a> {
    parent: HashMap<&'a str, &'a str>,
    rank: HashMap<&'a str, i32>,
}

impl<'a> UnionFind<'a> {
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    fn get_or_init(&mut self, node: &'a str) -> &'a str {
        self.parent.entry(node).or_insert(node);
        self.rank.entry(node).or_insert(0);
        node
    }

    fn find_root(&mut self, node: &'a str) -> &'a str {
        let parent = self.get_or_init(node);
        if parent != node {
            let root = self.find_root(parent);
            self.parent.insert(node, root);
            root
        } else {
            node
        }
    }

    fn union(&mut self, a: &'a str, b: &'a str) {
        let root_a = self.find_root(a);
        let root_b = self.find_root(b);
        if root_a != root_b {
            let rank_a = *self.rank.get(root_a).unwrap_or(&0);
            let rank_b = *self.rank.get(root_b).unwrap_or(&0);
            if rank_a < rank_b {
                self.parent.insert(root_a, root_b);
            } else if rank_a > rank_b {
                self.parent.insert(root_b, root_a);
            } else {
                self.parent.insert(root_a, root_b);
                self.rank.insert(root_b, rank_b + 1);
            }
        }
    }

    fn count_components(&mut self, data: &'a HashMap<String, Vec<String>>) -> i32 {
        let mut nodes = HashSet::new();
        for (city, neighbors) in data {
            nodes.insert(city.as_str());
            for neighbor in neighbors {
                nodes.insert(neighbor.as_str());
            }
        }

        for node in &nodes {
            self.get_or_init(node);
        }
        for (city, neighbors) in data {
            for neighbor in neighbors {
                if city != neighbor {
                    self.union(city, neighbor);
                }
            }
        }

        let mut roots = HashSet::with_capacity(nodes.len());
        for node in &nodes {
            roots.insert(self.find_root(node));
        }
        roots.len() as i32
    }
}

fn read_json(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn process_district_data() -> Result<String, Box<dyn std::error::Error>> {
    let json_str = read_json(DISTRICT_FILE)?;
    let raw_data: DistrictData = serde_json::from_str(&json_str)?;

    // 合并重复键
    let mut data: HashMap<String, HashMap<String, Vec<String>>> = HashMap::with_capacity(5);
    for (batch, inner) in raw_data {
        let entry = data.entry(batch).or_insert_with(|| HashMap::with_capacity(inner.len()));
        for (key, mut value) in inner {
            value.sort();
            value.dedup();
            let existing = entry.entry(key).or_insert_with(Vec::new);
            existing.extend(value);
            existing.sort();
            existing.dedup();
        }
    }

    // 用 Vec 按批次索引
    const MAX_BATCH: usize = 5;
    let mut batches = vec![None; MAX_BATCH];
    for (batch, districts) in data {
        if let Ok(index) = batch.parse::<usize>() {
            if index > 0 && index <= MAX_BATCH {
                batches[index - 1] = Some(districts);
            }
        }
    }

    // 计算连通分量
    let mut results = Vec::with_capacity(MAX_BATCH);
    let mut total_len = 0;
    for (i, opt_districts) in batches.iter().enumerate() {
        let mut uf = UnionFind::new();
        let count = match opt_districts {
            Some(districts) => {
                let c = uf.count_components(districts);
                println!("Batch {}: {}", i + 1, c);
                total_len += c.to_string().len() + 1;
                c
            }
            None => {
                println!("Batch {}: 0 (missing)", i + 1);
                total_len += 2;
                0
            }
        };
        results.push(count);
    }

    let mut output = String::with_capacity(total_len - 1);
    for (i, count) in results.iter().enumerate() {
        if i > 0 {
            output.push(',');
        }
        output.push_str(&count.to_string());
    }

    Ok(output)
}

pub fn count_provinces() -> String {
    process_district_data().unwrap_or_else(|_| "".to_string())
}