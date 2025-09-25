#[cfg(test)]
mod tests {
    use data_orchestra::variables::tree::VariableTree;
    use rstest::rstest;
    use serde_json::{Value, json};

    #[rstest]
    #[case(json!(1))]
    #[case(json!(true))]
    #[case(json!("String"))]
    #[case(json!({ "key": "value" }))]
    fn create_tree(#[case] root: Value) {
        let tree = VariableTree::new(root.clone());

        assert_eq!(tree.nodes.get(0).unwrap().value, root);
    }

    #[rstest]
    #[case(json!({ "key": 1 }), "key", json!(1))]
    #[case(json!({ "key": true }), "key", json!(true))]
    #[case(json!({ "key": "value" }), "key", json!("value"))]
    #[case(json!({ "key": { "x": "y" } }), "key", json!({ "x": "y" }))]
    fn add_child(#[case] root: Value, #[case] child_key: String, #[case] child_value: Value) {
        let mut tree = VariableTree::new(root);
        let id = tree.add_child(0, child_key, child_value.clone());

        assert_eq!(tree.nodes.get(id).unwrap().value, child_value);

        assert_eq!(tree.nodes.len(), 2);

        let children = &tree.get_root().unwrap().children;

        assert!(children.contains(&id));
    }

    #[test]
    fn correct_tree_children() {
        let root = json!({
            "1": { "1": "1" },
            "2": { "2": "2" },
            "3": { "3": "3" }
        });

        let mut tree = VariableTree::new(root);

        let sub_1 = tree.add_child(0, "1".to_string(), json!({ "1": "1" }));
        let sub_2 = tree.add_child(0, "2".to_string(), json!({ "2": "2" }));
        let sub_3 = tree.add_child(0, "3".to_string(), json!({ "3": "3" }));

        let sub_1_sub = tree.add_child(sub_1, "1".to_string(), json!("1"));
        let sub_2_sub = tree.add_child(sub_2, "2".to_string(), json!("2"));
        let sub_3_sub = tree.add_child(sub_3, "3".to_string(), json!("3"));

        assert_eq!(tree.nodes.len(), 7);

        assert_eq!(tree.get_root().unwrap().children.len(), 3);
        assert!(tree.get_root().unwrap().children.contains(&sub_1));
        assert!(tree.get_root().unwrap().children.contains(&sub_2));
        assert!(tree.get_root().unwrap().children.contains(&sub_3));

        let c_1 = tree.nodes.get(sub_1).unwrap();
        assert_eq!(c_1.children.len(), 1);
        assert!(c_1.children.contains(&sub_1_sub));

        let c_2 = tree.nodes.get(sub_2).unwrap();
        assert_eq!(c_2.children.len(), 1);
        assert!(c_2.children.contains(&sub_2_sub));

        let c_3 = tree.nodes.get(sub_3).unwrap();
        assert_eq!(c_3.children.len(), 1);
        assert!(c_3.children.contains(&sub_3_sub));
    }

    #[test]
    fn tree_get_value() {
        let root = json!({
            "1": { "2": "3" },
            "4": { "5": "6" },
            "7": { "8": "9" }
        });

        let mut tree = VariableTree::new(root);

        let sub_1 = tree.add_child(0, "1".to_string(), json!({ "2": "3" }));
        let sub_2 = tree.add_child(0, "4".to_string(), json!({ "5": "6" }));
        let sub_3 = tree.add_child(0, "7".to_string(), json!({ "8": "9" }));

        let _sub_1_sub = tree.add_child(sub_1, "2".to_string(), json!("3"));
        let _sub_2_sub = tree.add_child(sub_2, "5".to_string(), json!("6"));
        let _sub_3_sub = tree.add_child(sub_3, "8".to_string(), json!("9"));

        dbg!(&tree);

        let result = tree.get_variable_value(&"1".to_string());
        assert_eq!(result, Some(json!({ "2": "3" })));

        let result = tree.get_variable_value(&"4".to_string());
        assert_eq!(result, Some(json!({ "5": "6" })));

        let result = tree.get_variable_value(&"7".to_string());
        assert_eq!(result, Some(json!({ "8": "9" })));

        let result = tree.get_variable_value(&"1.2".to_string());
        assert_eq!(result, Some(json!("3")));

        let result = tree.get_variable_value(&"4.5".to_string());
        assert_eq!(result, Some(json!("6")));

        let result = tree.get_variable_value(&"7.8".to_string());
        assert_eq!(result, Some(json!("9")));
    }

    #[test]
    fn tree_invalid_get_value() {
        let root = json!({
            "1": { "2": "3" },
            "4": { "5": "6" },
            "7": { "8": "9" }
        });

        let mut tree = VariableTree::new(root);

        let sub_1 = tree.add_child(0, "1".to_string(), json!({ "2": "3" }));
        let sub_2 = tree.add_child(0, "4".to_string(), json!({ "5": "6" }));
        let sub_3 = tree.add_child(0, "7".to_string(), json!({ "8": "9" }));

        let _sub_1_sub = tree.add_child(sub_1, "2".to_string(), json!("3"));
        let _sub_2_sub = tree.add_child(sub_2, "5".to_string(), json!("6"));
        let _sub_3_sub = tree.add_child(sub_3, "8".to_string(), json!("9"));

        let result = tree.get_variable_value(&"2".to_string());
        assert_eq!(result, None);

        let result = tree.get_variable_value(&"5".to_string());
        assert_eq!(result, None);

        let result = tree.get_variable_value(&"8".to_string());
        assert_eq!(result, None);
    }
}
