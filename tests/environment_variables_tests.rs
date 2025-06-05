#[cfg(test)]
mod tests {
    use rstest::rstest;
    use data_orchestra::core::env::env::Env;
    use serde_json::{json, Value};

    #[rstest]
    #[case(
        vec![], 
        vec![], 
        ""
    )]
    #[case(
        vec!["BOOL"],
        vec![Value::Bool(true)], 
        r#"BOOL=true"#
    )]
    #[case(
        vec!["NUMBER"], 
        vec![serde_json::to_value(1).unwrap()], 
        "NUMBER=1"
    )]
    #[case(
        vec!["STRING"], 
        vec![Value::String("string".to_string())], 
        r#"STRING="string""#
    )]
    #[case(
        vec!["ARRAY"], 
        vec![Value::Array(vec![Value::String("value_1".to_string()), Value::String("value_2".to_string())])], 
        r#"ARRAY="["value_1","value_2"]""#
    )]
    #[case(
        vec!["MAP"],
        vec![json!({ "key": "value" })],
        r#"MAP="{"key":"value"}""#
    )]
    /*#[case(
        vec!["NUMBER", "BOOL"],
        vec![serde_json::to_value(1).unwrap() ,Value::Bool(true)], 
        "NUMBER=1\nBOOL=true"
    )]*/
    pub fn environment_parse(#[case] keys: Vec<&str>, #[case] values: Vec<Value>, #[case] expected: &str) {

        let mut env = Env::new();
        for (key, value) in keys.iter().zip(values) {
            env.add_mut(*key, value);
        }

        let env_string = env.parse();
        assert_eq!(env_string, expected);
    }
}
