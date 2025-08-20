

#[cfg(test)]
mod tests {
    use serde_json::json;
    use data_orchestra::interface::process::ExtProcess;

    pub fn get_process(json: serde_json::Value) -> ExtProcess {
        serde_json::from_value(json).expect("Unable to get process from json")
    }

    #[test]
    pub fn process_resource() {
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

        let process = get_process(json);
        assert!(process.object.resources.has_one());
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

        let process = get_process(json);
        assert!(process.object.resources.has_multiple());
        assert_eq!(process.object.resources.get_amount(), 2);
    }
}