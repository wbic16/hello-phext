#[cfg(test)]
mod tests {
    use crate::phext;

    #[test]
    fn test_coordinate_parsing() {
        let example_coordinate = "9.8.7/6.5.4/3.2.1";
        let test: phext::Coordinate = phext::to_coordinate(example_coordinate);
        let address = test.to_string();
        assert_eq!(address, example_coordinate, "Coordinate parsing failed");
    }

    #[test]
    fn test_scrolls() {
        let expect1 = "Hello World";
        let expect2 = "Scroll #2 -- this text will be selected";
        let expect3 = "Scroll #3 - this text will be ignored";

        let sample = format!("{expect1}\x17{expect2}\x17{expect3}");

        let coord1 = phext::to_coordinate("1.1.1/1.1.1/1.1.1");
        let coord2 = phext::to_coordinate("1.1.1/1.1.1/1.1.2");
        let coord3 = phext::to_coordinate("1.1.1/1.1.1/1.1.3");

        let text1 = phext::fetch(&sample, coord1);
        assert_eq!(text1, expect1, "Fetching text for coord1 failed");

        let text2 = phext::fetch(&sample, coord2);
        assert_eq!(text2, expect2, "Fetching text for coord2 failed");

        let text3 = phext::fetch(&sample, coord3);
        assert_eq!(text3, expect3, "Fetching text for coord3 failed");
    }
}