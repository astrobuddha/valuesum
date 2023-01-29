mod valuator_tests {
    use crate::valuator::Valuator;

    #[test]
    fn constructor_test() {
        let my_val = Valuator::new();

        assert!(true);
    }

    #[test]
    fn evaulate_when_letter_expect_value() {
        let my_val = Valuator::new();

        let input = "a";
        let expected: u32 = 1;

        assert_eq!(my_val.evaluate(input), expected);
    }

    #[test]
    fn evaluate_when_captial_expect_same_value() {
        let my_val = Valuator::new();

        let input = "A";
        let expected: u32 = 1;

        assert_eq!(my_val.evaluate(input), expected);
    }

    #[test]
    fn evaluate_when_multiple_lowercase_expect_value() {
        let my_val = Valuator::new();

        let input = "test";
        let expected: u32 = 64;

        assert_eq!(my_val.evaluate(input), expected);
    }

    #[test]
    fn evaluate_number_expect_zero() {
        let my_val = Valuator::new();

        let input = "5";
        let expected: u32 = 0;

        assert_eq!(my_val.evaluate(input), expected);
    }

    #[test]
    fn evaluate_special_character_expect_zero() {
        let my_val = Valuator::new();

        let input = ":";
        let expected: u32 = 0;

        assert_eq!(my_val.evaluate(input), expected);
    }
}
