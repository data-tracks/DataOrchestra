#[cfg(test)]
mod tests {
    use serde_json::json;
    use data_orchestra::core::object::Graph;
    use data_orchestra::interface::general::General;

    pub fn get_graph(json: serde_json::Value) -> Graph {
        serde_json::from_value(json).expect("Unable to parse json to graph")
    }

    #[test]
    pub fn graph_empty() {
        let json = json!({});

        let graph = get_graph(json);

        assert_eq!(graph.ignore, false);
        assert_eq!(graph.to, Vec::<String>::new());
    }

    #[test]
    pub fn graph_to() {
        let json = json!(
        {
            "to": [ "other" ]
        });

        let graph = get_graph(json);

        assert_eq!(graph.ignore, false);
        assert_eq!(graph.to, vec!["other".to_string()]);
    }

    #[test]
    pub fn graph_ignore() {
        let json = json!(
        {
            "ignore": true
        });

        let graph = get_graph(json);

        assert_eq!(graph.ignore, true);
        assert_eq!(graph.to, Vec::<String>::new());
    }

    #[test]
    pub fn graph() {
        let json = json!(
        {
            "ignore": true,
            "to": [ "other", "again_other" ]
        });

        let graph = get_graph(json);

        assert_eq!(graph.ignore, true);
        assert_eq!(graph.to, vec!["other".to_string(), "again_other".to_string()]);
    }
}