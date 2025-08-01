#[cfg(test)]
mod tests {
    use serde_json::json;
    use data_orchestra::interface::generate::ExtGenerate;

    pub fn get_generate(json: serde_json::Value) -> ExtGenerate {
        let generate: ExtGenerate = serde_json::from_value(json).expect("Unable to parse json to generate");
        generate
    }

    #[test]
    pub fn generate_template_type_default() {

        let json = json!(
        {
            "sensor": {
                "interval": 250,
                "address": "address",
            }
        });

        let generate = get_generate(json);
        assert!(generate.config.is_some());
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
        assert_eq!(generate.amount, 5);
    }

    #[test]
    pub fn generate_resource() {
        let json = json!(
        {
            "resources": {
                "type": "data",
                "location": "node",
                "name": "data",
                "source": "/source",
                "destination": "/destination"
            }
        });

        let generate = get_generate(json);
        assert!(generate.general.resources.has_one());
    }

    #[test]
    pub fn generate_resource_multiple() {
        let json = json!(
        {
            "resources": [
                {
                    "type": "data",
                    "location": "node",
                    "name": "data",
                    "source": "/source",
                    "destination": "/destination"
                },
                {
                    "type": "data",
                    "location": "node",
                    "name": "data",
                    "source": "/source",
                    "destination": "/destination"
                }
            ]
        });

        let generate = get_generate(json);
        assert!(generate.general.resources.has_multiple());
        assert_eq!(generate.general.resources.get_amount(), 2);
    }
}