

#[cfg(test)]
mod tests {
    use serde_json::json;
    use data_orchestra::interface::store::ExtStore;

    pub fn get_store(json: serde_json::Value) -> ExtStore {
        serde_json::from_value(json).expect("Unable to get store from json")
    }

    #[test]
    pub fn store_resource() {
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

        let store = get_store(json);
        assert!(store.general.resources.has_one());
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

        let store = get_store(json);
        assert!(store.general.resources.has_multiple());
        assert_eq!(store.general.resources.get_amount(), 2);
    }
}