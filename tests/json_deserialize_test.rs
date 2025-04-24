mod tests {
    use core::panic;

    use DataOrchester::docker::docker_struct::Container;

    #[test]
    fn test() {
        println!("{}", serde_json::to_string(&Container::new()).unwrap());
        panic!();
    }
}
