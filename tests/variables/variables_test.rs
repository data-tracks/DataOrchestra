#[cfg(test)]
mod tests {
    use rstest::rstest;
    use data_orchestra::variables::variables::Variables;

    #[rstest]
    #[case(
        r#"{ "variables": { "NAME": "someone" } }"#, 
        r#"{ "name": "${NAME}" }"#, 
        r#"{ "name": "someone" }"#
        )]
    #[case(
        r#"{ "variables": { "AGE": 23 } }"#, 
        r#"{ "age": "${AGE}" }"#, 
        r#"{ "age": 23 }"#
        )]
    #[case(
        r#"{ "variables": { "AGE": 23.5 } }"#, 
        r#"{ "age": "${AGE}" }"#, 
        r#"{ "age": 23.5 }"#
        )]
    #[case(
        r#"{ "variables": { "EXISTS": true } }"#, 
        r#"{ "exists": "${EXISTS}" }"#, 
        r#"{ "exists": true }"#
        )]
    #[case(
        r#"{ "variables": { "EXISTS": false } }"#, 
        r#"{ "exists": "${EXISTS}" }"#, 
        r#"{ "exists": false }"#
        )]
    #[case(
        r#"{ "variables": { "NAME": "someone", "COUNTRY": "CH" } }"#,
        r#"{ "name": "${NAME}", "country": "${COUNTRY}" }"#,
        r#"{ "name": "someone", "country": "CH" }"#
        )]
    pub fn single_variable(#[case] var: &str, #[case] config: &str, #[case] expected: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let result = variables.parse(config.to_string());

        let actual_json: serde_json::Value = serde_json::from_str(&result).expect("Failed to parse actual result as JSON");
        let expected_json: serde_json::Value = serde_json::from_str(expected).expect("Failed to parse expected result as JSON");

        assert_eq!(actual_json, expected_json);
    }

    #[rstest]
    #[case(
        r#"{ "variables": { "NAME": "someone" } }"#, 
        r#"{ "name": "${AGE}" }"#, 
        )]
    #[should_panic]
    pub fn single_invalid_variable(#[case] var: &str, #[case] config: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let _ = variables.parse(config.to_string());
    }


    #[rstest]
    #[case(
        r#"{ "variables": { "NAMES": ["someone", "someone else"] } }"#, 
        r#"{ "names": "${NAMES}" }"#, r#"{ "names": ["someone", "someone else"] }"#
        )]
    #[case(
        r#"{ "variables": { "NAMES": ["someone", "someone else"], "COUNTRIES": ["CH", "NL"] } }"#, 
        r#"{ "names": "${NAMES}", "countries": "${COUNTRIES}" }"#, 
        r#"{ "names": ["someone", "someone else"], "countries": ["CH", "NL"] }"#
        )]
    pub fn array_variable(#[case] var: &str, #[case] config: &str, #[case] expected: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let result = variables.parse(config.to_string());

        let actual_json: serde_json::Value = serde_json::from_str(&result).expect("Failed to parse actual result as JSON");
        let expected_json: serde_json::Value = serde_json::from_str(expected).expect("Failed to parse expected result as JSON");

        assert_eq!(actual_json, expected_json);
    }

    #[rstest]
    #[case(
        r#"{ "variables": { "PERSON": { "name": "someone", "country": "CH" } } }"#, 
        r#"{ "person": "${PERSON}" }"#, 
        r#"{ "person": { "name": "someone", "country": "CH" } }"#
        )]
    #[case(
        r#"{ "variables": { "PERSON": { "name": "someone", "country": "CH" }, "COUNTRY": { "short": "CH", "long": "Switzerland" } } }"#, 
        r#"{ "person": "${PERSON}", "country": "${COUNTRY}" }"#, 
        r#"{ "person": { "country": "CH", "name": "someone" }, "country": { "short": "CH", "long": "Switzerland" } }"#
        )]
    pub fn structure_variable(#[case] var: &str, #[case] config: &str, #[case] expected: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let result = variables.parse(config.to_string());
        
        let actual_json: serde_json::Value = serde_json::from_str(&result).expect("Failed to parse actual result as JSON");
        let expected_json: serde_json::Value = serde_json::from_str(expected).expect("Failed to parse expected result as JSON");

        assert_eq!(actual_json, expected_json);
    }

    #[rstest]
    #[case(
        r#"{ "variables": { "NAME": "someone", "COUNTRIES": ["CH", "NL"] } }"#, 
        r#"{ "name": "${NAME}", "countries": "${COUNTRIES}" }"#, 
        r#"{ "name": "someone", "countries": ["CH", "NL"] }"#)]
    #[case(
        r#"{ "variables": { "NAME": "someone", "PERSON": { "name": "someone", "country": "CH" } } }"#, 
        r#"{ "name": "${NAME}", "person": "${PERSON}" }"#, 
        r#"{ "name": "someone", "person": { "name": "someone", "country": "CH" } }"#
        )]
    #[case(
        r#"{ "variables": { "COUNTRIES": ["CH", "NL"], "PERSON": { "name": "someone", "country": "CH" } } }"#, 
        r#"{ "countries": "${COUNTRIES}", "person": "${PERSON}" }"#, 
        r#"{ "countries": ["CH", "NL"], "person": { "name": "someone", "country": "CH" } }"#
        )]
    #[case(
        r#"{ "variables": { "NAME": "someone", "COUNTRIES": ["CH", "NL"], "PERSON": { "name": "someone", "country": "CH" } } }"#, 
        r#"{ "name": "${NAME}", "countries": "${COUNTRIES}", "person": "${PERSON}" }"#, 
        r#"{ "name": "someone", "countries": ["CH", "NL"], "person": { "name": "someone", "country": "CH" } }"#
        )]
    pub fn different_variables(#[case] var: &str, #[case] config: &str, #[case] expected: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let result = variables.parse(config.to_string());
        
        let actual_json: serde_json::Value = serde_json::from_str(&result).expect("Failed to parse actual result as JSON");
        let expected_json: serde_json::Value = serde_json::from_str(expected).expect("Failed to parse expected result as JSON");

        assert_eq!(actual_json, expected_json);
    }
    
    #[rstest]
    #[case(
        r#"{ "variables": { "PERSON": { "name": "someone" } } }"#, 
        r#"{ "name": "${PERSON.name}" }"#,
        r#"{ "name": "someone" }"#
        )]
    #[case(
        r#"{ "variables": { "PERSON": { "name": "someone", "age": 23 } } }"#, 
        r#"{ "name": "${PERSON.name}", "age": "${PERSON.age}" }"#,
        r#"{ "name": "someone", "age": 23 }"#
        )]
    #[case(
        r#"{ "variables": { "PERSON": { "name": "someone", "countries": ["CH", "NL"] } } }"#, 
        r#"{ "name": "${PERSON.name}", "countries": "${PERSON.countries}" }"#,
        r#"{ "name": "someone", "countries": ["CH", "NL"] }"#
        )]
    #[case(
        r#"{ "variables": { "PERSON": { "name": "someone", "data": { "id": 1 } } } }"#, 
        r#"{ "name": "${PERSON.name}", "data": "${PERSON.data}" }"#,
        r#"{ "name": "someone", "data": { "id" : 1} }"#
        )]
    pub fn map_variable_access(#[case] var: &str, #[case] config: &str, #[case] expected: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let result = variables.parse(config.to_string());
        
        let actual_json: serde_json::Value = serde_json::from_str(&result).expect("Failed to parse actual result as JSON");
        let expected_json: serde_json::Value = serde_json::from_str(expected).expect("Failed to parse expected result as JSON");

        assert_eq!(actual_json, expected_json);
    }

    #[rstest]
    #[case(
        r#"{ "variables": { "PERSON": { "name": "someone" } } }"#, 
        r#"{ "name": "${PERSON.age}" }"#,
        )]
    #[should_panic]
        pub fn map_invalid_variable_access(#[case] var: &str, #[case] config: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let _ = variables.parse(config.to_string());
    }
 
    #[rstest]
    #[case(
        r#"{ "variables": { "PERSON": { "name": { "sirname": "someone" } } } }"#, 
        r#"{ "name": "${PERSON.name.sirname}" }"#,
        r#"{ "name": "someone" }"#
        )]
    pub fn map_variable_access_deep(#[case] var: &str, #[case] config: &str, #[case] expected: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let result = variables.parse(config.to_string());
        
        let actual_json: serde_json::Value = serde_json::from_str(&result).expect("Failed to parse actual result as JSON");
        let expected_json: serde_json::Value = serde_json::from_str(expected).expect("Failed to parse expected result as JSON");

        assert_eq!(actual_json, expected_json);
    }

    #[rstest]
    #[case(
        r#"{ "variables": { "PERSON": { "name": { "sirname": "someone" } } } }"#, 
        r#"{ "name": "${PERSON.name.age}" }"#,
        )]
    #[should_panic]
    pub fn map_invalid_variable_access_deep(#[case] var: &str, #[case] config: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let _ = variables.parse(config.to_string());
    }

    #[rstest]
    #[case(
        r#"{ "variables": { "NAME": "someone" } }"#, 
        r#"{ "name": "name is ${NAME}" }"#, 
        r#"{ "name": "name is someone" }"#
        )]
    #[case(
        r#"{ "variables": { "AGE": 23 } }"#, 
        r#"{ "age": "age is ${AGE}" }"#, 
        r#"{ "age": "age is 23" }"#
        )]
    #[case(
        r#"{ "variables": { "AGE": 23.5 } }"#, 
        r#"{ "age": "age is ${AGE}" }"#, 
        r#"{ "age": "age is 23.5" }"#
        )]
    #[case(
        r#"{ "variables": { "EXISTS": true } }"#, 
        r#"{ "exists": "exists? ${EXISTS}" }"#, 
        r#"{ "exists": "exists? true" }"#
        )]
    #[case(
        r#"{ "variables": { "EXISTS": false } }"#, 
        r#"{ "exists": "exists? ${EXISTS}" }"#, 
        r#"{ "exists": "exists? false" }"#
        )]
    fn variable_access_inside_string(#[case] var: &str, #[case] config: &str, #[case] expected: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let result = variables.parse(config.to_string());
        
        let actual_json: serde_json::Value = serde_json::from_str(&result).expect("Failed to parse actual result as JSON");
        let expected_json: serde_json::Value = serde_json::from_str(expected).expect("Failed to parse expected result as JSON");

        assert_eq!(actual_json, expected_json);
    }

    #[rstest]
    #[case(
        r#"{ "variables": { "NAMES": ["someone", "someone_else"] } }"#,
        r#"{ "name": "${NAMES.0}" }"#,
        r#"{ "name": "someone" }"#
    )]
    #[case(
        r#"{ "variables": { "NAMES": ["someone", "someone_else"] } }"#,
        r#"{ "name": "${NAMES.1}" }"#,
        r#"{ "name": "someone_else" }"#
    )]
    fn array_variables_index(#[case] var: &str, #[case] config: &str, #[case] expected: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let result = variables.parse(config.to_string());
       
        let actual_json: serde_json::Value = serde_json::from_str(&result).expect("Failed to parse actual result as JSON");
        let expected_json: serde_json::Value = serde_json::from_str(expected).expect("Failed to parse expected result as JSON");

        assert_eq!(actual_json, expected_json);
    }

    #[rstest]
    #[case(
        r#"{ "variables": { "NAMES": ["someone", "someone_else"] } }"#,
        r#"{ "name": "${NAMES.2}" }"#,
    )]
    #[should_panic]
    fn array_variable_invalid_index(#[case] var: &str, #[case] config: &str) {
        let variables: Variables = serde_json::from_str(var).expect("Unable to parse variables to struct");
        let _ = variables.parse(config.to_string());
    }
}
