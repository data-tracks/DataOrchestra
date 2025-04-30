mod tests {

    #[test]
    fn test() {
        let compose = Path::new(self.compose.as_ref().unwrap());
        let yaml = fs::read_to_string(compose).expect("Unable to read compose");
        YamlLoader::load_from_str(yaml.as_str());
    }
}
