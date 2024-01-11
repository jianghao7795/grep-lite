pub mod json_serde {
    use serde::Deserialize;
    use serde_json::Result;

    #[derive(Debug, Deserialize)]
    pub struct MyData {
        /*外部使用 必须pub */
        key1: String,
        key2: i32,
        // Add other fields as needed
    }

    pub fn json_to_data() -> Result<MyData> {
        let json_data = r#"
            {
                "key1": "value1",
                "key2": 42
            }
        "#;
        // unwrap_or_else 返回默认值
        let parsed_data: MyData = serde_json::from_str(json_data).unwrap_or_else(|e| {
            println!("{:?}", e);
            return MyData {
                key1: "test".to_string(),
                key2: 1,
            };
        });
        println!("{}", parsed_data.key1);
        println!("{}", parsed_data.key2);
        Ok(parsed_data)
    }
}
