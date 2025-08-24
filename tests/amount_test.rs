#[cfg(test)]
mod amount {
    use rstest::rstest;
    use data_orchestra::shared::Amount;

    #[test]
    pub fn amount_none() {
        let nothing: Amount<usize> = Amount::None;

        assert!(nothing.has_none());

        assert!(!nothing.has_something());
        assert!(!nothing.has_one());
        assert!(!nothing.has_multiple());
    }

    #[test]
    pub fn amount_single() {
        let single = Amount::Single(1);

        assert!(single.has_one());
        assert!(single.has_something());

        assert!(!single.has_none());
        assert!(!single.has_multiple());
    }

    #[test]
    pub fn amount_multiple() {
        let single = Amount::Multiple(vec![1, 2]);

        assert!(single.has_multiple());
        assert!(single.has_something());

        assert!(!single.has_none());
        assert!(!single.has_one());
    }

    #[rstest]
    #[case(Amount::None, vec![])]
    #[case(Amount::Single(1), vec![1])]
    #[case(Amount::Multiple(vec![1, 2]), vec![1, 2])]
    #[case(Amount::Multiple(vec![1, 2, 3]), vec![1, 2, 3])]
    pub fn amount_to_vec(#[case] items: Amount<usize>, #[case] amount: Vec<usize>) {
        assert_eq!(items.to_vec(),  amount);
    }

    #[rstest]
    #[case(Amount::None, 0)]
    #[case(Amount::Single(1), 1)]
    #[case(Amount::Multiple(vec![1, 2]), 2)]
    #[case(Amount::Multiple(vec![1, 2, 3]), 3)]
    pub fn amount_get_amount(#[case] items: Amount<usize>, #[case] amount: usize) {
        assert_eq!(items.get_amount(),  amount);
    }

    #[rstest]
    #[case(Amount::None, 1, Amount::Single(1))]
    #[case(Amount::Single(1), 2, Amount::Multiple(vec![1, 2]))]
    #[case(Amount::Multiple(vec![1, 2]), 3, Amount::Multiple(vec![1, 2, 3]))]
    pub fn amount_insert_single(#[case] mut amount: Amount<usize>, #[case] item: usize, #[case] result: Amount<usize>) {
        amount.insert(item);
        assert_eq!(amount, result);
    }
}