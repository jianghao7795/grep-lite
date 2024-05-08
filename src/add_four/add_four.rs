use std::collections::HashMap;

// #[allow(dead_code)] // 允许未使用的变量及函数
pub fn hashmap() {
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, team.1);
    }

    println!("{:#?}", teams_map);
    let searcher = String::from("日本队");
    let score = teams_map.get(&searcher);
    println!("{} 的得分是 {:?}", searcher, score);
    match score {
        Some(score) => println!("{} 的得分是 {}", searcher, score),
        None => println!("{} 没有参加比赛", searcher),
    }
}

pub fn add_four(base: u32) -> u32 {
    base + 4
}
