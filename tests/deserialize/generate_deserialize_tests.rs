#[cfg(test)]
mod tests {
    use serde_json::json;
    use data_orchestra::interface::generate::ExtGenerate;
    use data_orchestra::shared::ToInternalVec;

    pub fn get_generate(json: serde_json::Value) -> ExtGenerate {
        let generate: ExtGenerate = serde_json::from_value(json).expect("Unable to parse json to generate");
        generate
    }

    #[test]
    pub fn generate_template_type_default() {

        let json = json!(
        {
            "template": {
                "type": "sensor"
            }
        });

        let generate = get_generate(json);
        assert!(generate.template.is_some());
    }

    #[test]
    pub fn name_set_test() {
        let json = json!(
        {
            "name": "GENERATE"
        });

        let generate = get_generate(json);
        assert_eq!(generate.general.name, Some("GENERATE".to_string()));
    }

    #[test]
    pub fn generate_amount() {
        let json = json!(
        {
            "name": "GENERATE",
            "amount": 5
        });

        let generate = get_generate(json);
        let int_generate = generate.to_internal();

        assert_eq!(int_generate.len(), 5);
    }
}