#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::{json, Map};
    use tokio_postgres::types::ToSql;
    use data_orchestra::interface::data::{ExtData, ExtDataTypes, ExtVolatile, VolatileTypes};
    use data_orchestra::interface::location::Location;

    pub fn get_data_type(json: serde_json::Value) -> ExtDataTypes {
        let data_type: ExtDataTypes = serde_json::from_value(json).expect("Unable to parse json to DataTypes");
        data_type
    }

    #[test]
    #[should_panic]
    pub fn no_type() {
        let json = json!(
        {
            "name": "NAME"
        });

        let _ = get_data_type(json);
    }

    #[test]
    #[should_panic]
    pub fn data_no_path() {
        let json = json!(
        {
            "type": "data",
            "location": "node",
            "name": "data",
            "destination": "/destination"
        });

        let _ = get_data_type(json);
    }

    #[test]
    #[should_panic]
    pub fn data_no_path_no_destination() {
        let json = json!(
        {
            "type": "data",
            "location": "node",
            "name": "data",
        });

        let _ = get_data_type(json);
    }

    #[test]
    #[should_panic]
    pub fn data_no_destination() {
        let json = json!(
        {
            "type": "data",
            "location": "node",
            "name": "data",
            "path": "/path"
        });

        let _ = get_data_type(json);
    }

    #[test]
    pub fn data() {
        let json = json!(
        {
            "type": "data",
            "location": "node",
            "name": "data",
            "path": "/path",
            "destination": "/destination"
        });

        let data = get_data_type(json);
        let data_struct = ExtData { location: Location::Node, name: Some("data".to_string()), path: "/path".to_string(), destination: "/destination".to_string(), dependency: None};
        assert_eq!(data.get_data_ref(), &data_struct);
    }

    #[test]
    #[should_panic]
    pub fn volatile_no_destination() {
        let json = json!(
        {
            "type": "volatile",
            "location": "node",
            "name": "data",
        });

        let _ = get_data_type(json);
    }

    #[test]
    #[should_panic]
    pub fn volatile_nothing() {
        let json = json!(
        {
            "type": "volatile",
            "location": "node",
            "name": "data",
            "destination": "/destination"
        });

        let _ = get_data_type(json);
    }

    #[test]
    pub fn volatile_content() {
        let json = json!(
        {
            "type": "volatile",
            "location": "node",
            "name": "data",
            "path": "/path",
            "destination": "/destination",
            "content": "CONTENT"
        });

        let data = get_data_type(json);
        let volatile = ExtVolatile { location: Location::Node, name: Some("data".to_string()), destination: "/destination".to_string(), volatile_types: VolatileTypes::Content("CONTENT".to_string())};
        assert_eq!(data.get_volatile_ref(), &volatile);
    }

    #[test]
    pub fn volatile_json() {
        let json = json!(
        {
            "type": "volatile",
            "location": "node",
            "name": "data",
            "path": "/path",
            "destination": "/destination",
            "json": {
                "KEY": "VALUE",
                "MAP": {
                    "KEY": "VALUE"
                }
            }
        });


        let data = get_data_type(json);
        let map = json!(
        {
            "KEY": "VALUE",
            "MAP": {
                "KEY": "VALUE"
            }
        }).as_object().unwrap().to_owned();
        let volatile = ExtVolatile { location: Location::Node, name: Some("data".to_string()), destination: "/destination".to_string(), volatile_types: VolatileTypes::Json(map)};
        assert_eq!(data.get_volatile_ref(), &volatile);
    }

    #[test]
    pub fn volatile_env() {
        let json = json!(
        {
            "type": "volatile",
            "location": "node",
            "name": "data",
            "path": "/path",
            "destination": "/destination",
            "env": {
                "KEY_1": "VALUE_1",
                "KEY_2": "VALUE_2"
            }
        });

        let data = get_data_type(json);
        let map = HashMap::from([("KEY_1".to_string(), "VALUE_1".to_string()), ("KEY_2".to_string(), "VALUE_2".to_string())]);
        let volatile = ExtVolatile { location: Location::Node, name: Some("data".to_string()), destination: "/destination".to_string(), volatile_types: VolatileTypes::Env(map)};
        assert_eq!(data.get_volatile_ref(), &volatile);
    }

    #[test]
    pub fn test() {
        let map = HashMap::from([("KEY_1".to_string(), "VALUE_1".to_string()), ("KEY_2".to_string(), "VALUE_2".to_string())]);
        let data_struct = ExtVolatile { location: Location::Node, name: Some("data".to_string()), destination: "/destination".to_string(), volatile_types: VolatileTypes::Env(map)};
        let json = serde_json::to_string_pretty(&data_struct).expect("");
        println!("{json}");
    }
}