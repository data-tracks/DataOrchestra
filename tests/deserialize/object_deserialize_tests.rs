#[cfg(test)]
mod tests {
    use data_orchestra::interface::ext_object::ExtObject;
    use serde_json::json;

    pub fn get_object(json: serde_json::Value) -> ExtObject {
        serde_json::from_value(json).expect("Unable to get object from json")
    }

    #[test]
    pub fn object_resource() {
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

        let object = get_object(json);
        assert!(object.general.resources.has_one());
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

        let object = get_object(json);
        assert!(object.general.resources.has_multiple());
        assert_eq!(object.general.resources.get_amount(), 2);
    }
}
