#[cfg(test)]
mod tests {
    use crate::phext::{self, LIBRARY_BREAK, SHELF_BREAK, SERIES_BREAK, COLLECTION_BREAK, VOLUME_BREAK, BOOK_BREAK, CHAPTER_BREAK, SECTION_BREAK, SCROLL_BREAK};
    use std::collections::HashMap;

    #[test]
    fn test_coordinate_parsing() {
        let example_coordinate: &str = "9.8.7/6.5.4/3.2.1";
        let test: phext::Coordinate = phext::to_coordinate(example_coordinate);
        let address: String = test.to_string();
        assert_eq!(address, example_coordinate, "Coordinate parsing failed");
    }

    fn test_helper(delim_in: u8, data: HashMap<&str, &str>) -> bool {
        let mut index: i32 = 0;
        let mut expect1: &str = "not set";
        let mut expect2: &str = "not set";
        let mut expect3: &str = "not set";
        let mut address1: &str = "not set";
        let mut address2: &str = "not set";
        let mut address3: &str = "not set";
        for x in data.keys() {
            if index == 0 { expect1 = x; address1 = data[x]; index += 1; }
            if index == 1 { expect2 = x; address2 = data[x]; index += 1; }
            if index == 2 { expect3 = x; address3 = data[x]; index += 1; }
        }
        if index < 3 { return false; }

        let buf: Vec<u8> = vec![delim_in];
        let delim: &str = std::str::from_utf8(&buf).unwrap();
        let sample: String = format!("{expect1}{delim}{expect2}{delim}{expect3}");

        let coord1: phext::Coordinate = phext::to_coordinate(address1);
        let coord2: phext::Coordinate = phext::to_coordinate(address2);
        let coord3: phext::Coordinate = phext::to_coordinate(address3);

        let text1: String = phext::fetch(&sample, coord1);
        assert_eq!(text1, expect1, "Fetching text for coord1 failed");

        let text2: String = phext::fetch(&sample, coord2);
        assert_eq!(text2, expect2, "Fetching text for coord2 failed");

        let text3: String = phext::fetch(&sample, coord3);
        assert_eq!(text3, expect3, "Fetching text for coord3 failed");

        return true;
    }

    #[test]
    fn test_scrolls() {
        let mut data: HashMap<&str, &str> = std::collections::HashMap::new();
        data.insert("Hello World", "1.1.1/1.1.1/1.1.1");
        data.insert("Scroll #2 -- this text will be selected", "1.1.1/1.1.1/1.1.2");
        data.insert("Scroll #3 - this text will be ignored", "1.1.1/1.1.1/1.1.3");

        let result: bool = test_helper(phext::SCROLL_BREAK as u8, data);
        assert_eq!(result, true);
    }

    #[test]
    fn test_sections() {
        let mut data: HashMap<&str, &str> = std::collections::HashMap::new();
        data.insert("Section A", "1.1.1/1.1.1/1.1.1");
        data.insert("Section B", "1.1.1/1.1.1/1.2.1");
        data.insert("Section C", "1.1.1/1.1.1/1.3.1");

        let result: bool = test_helper(phext::SECTION_BREAK as u8, data);
        assert_eq!(result, true);
    }

    #[test]
    fn test_chapters() {
        let mut data: HashMap<&str, &str> = std::collections::HashMap::new();
        data.insert("Chapter Alpha", "1.1.1/1.1.1/1.1.1");
        data.insert("Chapter Beta", "1.1.1/1.1.1/2.1.1");
        data.insert("Chapter Gamma", "1.1.1/1.1.1/3.1.1");

        let result: bool = test_helper(phext::CHAPTER_BREAK as u8, data);
        assert_eq!(result, true);
    }

    #[test]
    fn test_books() {
        let mut data: HashMap<&str, &str> = std::collections::HashMap::new();
        data.insert("Book z1", "1.1.1/1.1.1/1.1.1");
        data.insert("Book Something Else #2", "1.1.1/1.1.2/1.1.1");
        data.insert("Book Part 3", "1.1.1/1.1.3/1.1.1");

        let result: bool = test_helper(phext::BOOK_BREAK as u8, data);
        assert_eq!(result, true);
    }

    #[test]
    fn test_volumes() {
        let mut data: HashMap<&str, &str> = std::collections::HashMap::new();
        data.insert("Volume 1-1-1", "1.1.1/1.1.1/1.1.1");
        data.insert("Volume 1-2-1", "1.1.1/1.2.1/1.1.1");
        data.insert("Volume 1-3-1", "1.1.1/1.3.1/1.1.1");

        let result: bool = test_helper(phext::VOLUME_BREAK as u8, data);
        assert_eq!(result, true);
    }

    #[test]
    fn test_collections() {
        let mut data: HashMap<&str, &str> = std::collections::HashMap::new();
        data.insert("Collection 1-1-1", "1.1.1/1.1.1/1.1.1");
        data.insert("Collection 2-1-1", "1.1.1/2.1.1/1.1.1");
        data.insert("Collection 3-1-1", "1.1.1/3.1.1/1.1.1");

        let result: bool = test_helper(phext::COLLECTION_BREAK as u8, data);
        assert_eq!(result, true);
    }

    #[test]
    fn test_series() {
        let mut data: HashMap<&str, &str> = std::collections::HashMap::new();
        data.insert("Series 1-1-1", "1.1.1/1.1.1/1.1.1");
        data.insert("Series 1-1-2", "1.1.2/1.1.1/1.1.1");
        data.insert("Series 1-1-3", "1.1.3/1.1.1/1.1.1");

        let result: bool = test_helper(phext::SERIES_BREAK as u8, data);
        assert_eq!(result, true);
    }

    #[test]
    fn test_shelves() {
        let mut data: HashMap<&str, &str> = std::collections::HashMap::new();
        data.insert("Shelf 1-1-1", "1.1.1/1.1.1/1.1.1");
        data.insert("Shelf 1-2-1", "1.2.1/1.1.1/1.1.1");
        data.insert("Shelf 1-3-1", "1.3.1/1.1.1/1.1.1");

        let result: bool = test_helper(phext::SHELF_BREAK as u8, data);
        assert_eq!(result, true);
    }

    #[test]
    fn test_libraries() {
        let mut data: HashMap<&str, &str> = std::collections::HashMap::new();
        data.insert("Library 1-1-1", "1.1.1/1.1.1/1.1.1");
        data.insert("Library 2-1-1", "2.1.1/1.1.1/1.1.1");
        data.insert("Library 3-1-1", "3.1.1/1.1.1/1.1.1");

        let result = test_helper(phext::LIBRARY_BREAK as u8, data);
        assert_eq!(result, true);
    }

    #[test]
    fn test_coordinates_invalid() {
        let c1: phext::Coordinate = phext::to_coordinate("0.0.0/0.0.0/0.0.0"); // invalid
        let c2 = phext::Coordinate {
            z: phext::ZCoordinate{library: 0, shelf: 0, series: 0},
            y: phext::YCoordinate{collection: 0, volume: 0, book: 0},
            x: phext::XCoordinate{chapter: 0, section: 0, scroll: 0}};
        assert_eq!(c1, c2);
        let c1b: bool = c1.validate_coordinate();
        let c2b: bool = c2.validate_coordinate();
        assert_eq!(c1b, false);
        assert_eq!(c2b, false);
    }

    #[test]
    fn test_coordinates_valid() {
        let c1: phext::Coordinate = phext::to_coordinate("255.254.253/32.4.8/4.2.1"); // valid
        let c2 = phext::Coordinate {
            z: phext::ZCoordinate{library: 255, shelf: 254, series: 253},
            y: phext::YCoordinate{collection: 32, volume: 4, book: 8},
            x: phext::XCoordinate{chapter: 4, section: 2, scroll: 1}};
        assert_eq!(c1, c2);
        assert_eq!(c1.y.volume, 4);
        let c1b: bool = c1.validate_coordinate();
        let c2b: bool = c2.validate_coordinate();
        assert_eq!(c1b, false);
        assert_eq!(c2b, false);
    }

    #[test]
    fn test_url_encoding() {
        let c1: phext::Coordinate = phext::to_coordinate("142.143.144;145.146.147;148.149.150"); // valid
        let c2 = phext::Coordinate {
            z: phext::ZCoordinate{library: 142, shelf: 143, series: 144},
            y: phext::YCoordinate{collection: 145, volume: 146, book: 147},
            x: phext::XCoordinate{chapter: 148, section: 149, scroll: 150}};
        assert_eq!(c1, c2);
        let c1b: bool = c1.validate_coordinate();
        let c2b: bool = c2.validate_coordinate();
        assert_eq!(c1b, false);
        assert_eq!(c2b, false);
    }

    #[test]
    fn test_realistic_parse() {
        let coord: phext::Coordinate = phext::to_coordinate("6.13.4/2.11.4/2.20.3");
        let subspace = "here's some text at 6.13.4/2.11.4/2.20.3this is the next scroll and won't be picked";
        let result = phext::fetch(subspace, coord);
        assert_eq!(result, "here's some text at 6.13.4/2.11.4/2.20.3");
    }

    #[test]
    fn test_dead_reckoning() {        
        let mut test: String = "".to_string();
        test += "random text in 1.1.1/1.1.1/1.1.1 that we can skip past";
        test.push(LIBRARY_BREAK);
        test += "everything in here is at 2.1.1/1.1.1/1.1.1";
        test.push(SCROLL_BREAK);
        test += "and now we're at 2.1.1/1.1.1/1.1.2";
        test.push(SCROLL_BREAK);
        test += "moving on up to 2.1.1/1.1.1/1.1.3";
        test.push(BOOK_BREAK);
        test += "and now over to 2.1.1/1.1.2/1.1.1";
        test.push(SHELF_BREAK);
        test += "woot, up to 2.2.1/1.1.1/1.1.1";
        test.push(LIBRARY_BREAK);
        test += "here we are at 3.1.1/1.1.1.1.1";
        test.push(LIBRARY_BREAK); // 4.1.1/1.1.1/1.1.1
        test.push(LIBRARY_BREAK); // 5.1.1/1.1.1/1.1.1
        test += "getting closer to our target now 5.1.1/1.1.1/1.1.1";
        test.push(SHELF_BREAK); // 5.2.1
        test.push(SHELF_BREAK); // 5.3.1
        test.push(SHELF_BREAK); // 5.4.1
        test.push(SHELF_BREAK); // 5.5.1
        test.push(SERIES_BREAK); // 5.5.2
        test.push(SERIES_BREAK); // 5.5.3
        test.push(SERIES_BREAK); // 5.5.4
        test.push(SERIES_BREAK); // 5.5.5
        test += "here we go! 5.5.5/1.1.1/1.1.1";
        test.push(COLLECTION_BREAK); // 5.5.5/2.1.1/1.1.1
        test.push(COLLECTION_BREAK); // 5.5.5/3.1.1/1.1.1
        test.push(COLLECTION_BREAK); // 5.5.5/4.1.1/1.1.1
        test.push(BOOK_BREAK); // 5.5.5/4.1.2/1.1.1
        test.push(BOOK_BREAK); // 5.5.5/4.1.3/1.1.1
        test.push(BOOK_BREAK); // 5.5.5/4.1.4/1.1.1
        test += "this test appears at 5.5.5/4.1.4/1.1.1";
        test.push(VOLUME_BREAK); // 5.5.5/4.2.1/1.1.1
        test.push(VOLUME_BREAK); // 5.5.5/4.3.1/1.1.1
        test.push(VOLUME_BREAK); // 5.5.5/4.4.1/1.1.1
        test.push(VOLUME_BREAK); // 5.5.5/4.5.1/1.1.1
        test.push(VOLUME_BREAK); // 5.5.5/4.6.1/1.1.1
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.1/2.1.1
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.1/3.1.1
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.1/4.1.1
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.1/5.1.1
        test.push(BOOK_BREAK); // 5.5.5/4.6.2/1.1.1
        test.push(BOOK_BREAK); // 5.5.5/4.6.3/1.1.1
        test.push(BOOK_BREAK); // 5.5.5/4.6.4/1.1.1
        test.push(BOOK_BREAK); // 5.5.5/4.6.5/1.1.1
        test.push(BOOK_BREAK); // 5.5.5/4.6.6/1.1.1
        test.push(BOOK_BREAK); // 5.5.5/4.6.7/1.1.1
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.7/2.1.1
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.7/3.1.1
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.7/4.1.1
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.7/5.1.1
        test.push(SCROLL_BREAK); // 5.5.5/4.6.7/5.1.2
        test.push(SCROLL_BREAK); // 5.5.5/4.6.7/5.1.3
        test.push(SCROLL_BREAK); // 5.5.5/4.6.7/5.1.4
        test.push(SCROLL_BREAK); // 5.5.5/4.6.7/5.1.5
        test.push(SCROLL_BREAK); // 5.5.5/4.6.7/5.1.6
        test += "here's a test at 5.5.5/4.6.7/5.1.6";
        test.push(SCROLL_BREAK); // 5.5.5/4.6.7/5.1.7
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.7/6.1.1
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.7/7.1.1
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.7/8.1.1
        test.push(CHAPTER_BREAK); // 5.5.5/4.6.7/9.1.1
        test.push(SECTION_BREAK); // 5.5.5/4.6.7/9.2.1
        test.push(SECTION_BREAK); // 5.5.5/4.6.7/9.3.1
        test.push(SECTION_BREAK); // 5.5.5/4.6.7/9.4.1
        test.push(SECTION_BREAK); // 5.5.5/4.6.7/9.5.1
        test.push(SCROLL_BREAK);  // 5.5.5/4.6.7/9.5.2
        test.push(SCROLL_BREAK);  // 5.5.5/4.6.7/9.5.3
        test.push(SCROLL_BREAK);  // 5.5.5/4.6.7/9.5.4
        test.push(SCROLL_BREAK);  // 5.5.5/4.6.7/9.5.5
        test.push(SCROLL_BREAK);  // 5.5.5/4.6.7/9.5.6
        test.push(SCROLL_BREAK);  // 5.5.5/4.6.7/9.5.7
        test.push(SCROLL_BREAK);  // 5.5.5/4.6.7/9.5.8
        test.push(SCROLL_BREAK);  // 5.5.5/4.6.7/9.5.9
        test += "Expected Test Pattern Alpha Whisky Tango Foxtrot";
        let coord: phext::Coordinate = phext::to_coordinate("5.5.5/4.6.7/9.5.9");
        let result = phext::fetch(&test, coord);
        assert_eq!(result, "Expected Test Pattern Alpha Whisky Tango Foxtrot");

        let coord2 = phext::to_coordinate("5.5.5/4.6.7/5.1.6");
        let result2 = phext::fetch(&test, coord2);
        assert_eq!(result2, "here's a test at 5.5.5/4.6.7/5.1.6");
    }

    #[test]
    fn test_line_break() {
        assert_eq!(phext::LINE_BREAK, '\n');
    }

    #[test]
    fn test_more_cowbell() {
        assert_eq!(phext::MORE_COWBELL, '\x07');
    }

    #[test]
    fn test_coordinate_based_insert() {
        let mut test: String = "".to_string();
        test += "aaa";               // 1.1.1/1.1.1/1.1.1
        test.push(LIBRARY_BREAK); // 2.1.1/1.1.1/1.1.1
        test += "bbb";               // 
        test.push(SCROLL_BREAK);  // 2.1.1/1.1.1/1.1.2
        test += "ccc";

        // append 'ddd' after 'ccc'
        let root = phext::to_coordinate("1.1.1/1.1.1/1.1.1");
        let coord1 = phext::to_coordinate("2.1.1/1.1.1/1.1.3");
        let expected1 = phext::get_subspace_coordinates(test.as_bytes(), coord1);
        assert_eq!(expected1.2.z.library, 2);
        assert_eq!(expected1.2.z.shelf, 1);
        assert_eq!(expected1.2.z.series, 1);
        assert_eq!(expected1.2.y.collection, 1);
        assert_eq!(expected1.2.y.volume, 1);
        assert_eq!(expected1.2.y.book, 1);
        assert_eq!(expected1.2.x.chapter, 1);
        assert_eq!(expected1.2.x.section, 1);
        assert_eq!(expected1.2.x.scroll, 2);
        assert_eq!(expected1.0, 10);
        assert_eq!(expected1.1, 11);

        let mut expected_coord = phext::default_coordinate();
        expected_coord.z.library = 2;
        expected_coord.x.scroll = 3;
        assert_eq!(coord1, expected_coord);

        let update1 = phext::insert(&test, coord1, "ddd");
        assert_eq!(update1, "aaa\x01bbb\x17ccc\x17ddd");

        // append 'eee' after 'ddd'
        let coord2 = phext::to_coordinate("2.1.1/1.1.1/1.1.4");
        let update2 = phext::insert(update1.as_str(), coord2, "eee");
        assert_eq!(update2, "aaa\x01bbb\x17ccc\x17ddd\x17eee");

        // append 'fff' after 'eee'
        let coord3 = phext::to_coordinate("2.1.1/1.1./1.2.1");
        let update3 = phext::insert(update2.as_str(), coord3, "fff");
        assert_eq!(update3, "aaa\x01bbb\x17ccc\x17ddd\x17eee\x18fff");

        // append 'ggg' after 'fff'
        let coord4 = phext::to_coordinate("2.1.1/1.1.1/1.2.2");
        let update4 = phext::insert(update3.as_str(), coord4, "ggg");
        assert_eq!(update4, "aaa\x01bbb\x17ccc\x17ddd\x17eee\x18fff\x17ggg");

        // append 'hhh' after 'ggg'
        let coord5 = phext::to_coordinate("2.1.1/1.1.1/2.1.1");
        let update5 = phext::insert(update4.as_str(), coord5, "hhh");
        assert_eq!(update5, "aaa\x01bbb\x17ccc\x17ddd\x17eee\x18fff\x17ggg\x19hhh");

        // append 'iii' after 'eee'
        let coord6 = phext::to_coordinate("2.1.1/1.1.1/1.1.5");
        let update6 = phext::insert(update5.as_str(), coord6, "iii");
        assert_eq!(update6, "aaa\x01bbb\x17ccc\x17ddd\x17eee\x17iii\x18fff\x17ggg\x19hhh");

        // extend 1.1.1/1.1.1/1.1.1 with '---AAA'
        let update7 = phext::insert(update6.as_str(), root, "---AAA");
        assert_eq!(update7, "aaa---AAA\x01bbb\x17ccc\x17ddd\x17eee\x17iii\x18fff\x17ggg\x19hhh");

        // extend 2.1.1/1.1.1/1.1.1 with '---BBB'
        let coord8 = phext::to_coordinate("2.1.1/1.1.1/1.1.1");
        let update8 = phext::insert(update7.as_str(), coord8, "---BBB");
        assert_eq!(update8, "aaa---AAA\x01bbb---BBB\x17ccc\x17ddd\x17eee\x17iii\x18fff\x17ggg\x19hhh");

        // extend 2.1.1/1.1.1/1.1.2 with '---CCC'
        let coord9 = phext::to_coordinate("2.1.1/1.1.1/1.1.2");
        let update9 = phext::insert(update8.as_str(), coord9, "---CCC");
        assert_eq!(update9, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd\x17eee\x17iii\x18fff\x17ggg\x19hhh");

        // extend 2.1.1/1.1.1/1.1.3 with '---DDD'
        let coord10 = phext::to_coordinate("2.1.1/1.1.1/1.1.3");
        let update10 = phext::insert(update9.as_str(), coord10, "---DDD");
        assert_eq!(update10, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee\x17iii\x18fff\x17ggg\x19hhh");

        // extend 2.1.1/1.1.1/1.1.4 with '---EEE'
        let coord11 = phext::to_coordinate("2.1.1/1.1.1/1.1.4");
        let update11 = phext::insert(update10.as_str(), coord11, "---EEE");
        assert_eq!(update11, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee---EEE\x17iii\x18fff\x17ggg\x19hhh");

        // extend 2.1.1/1.1.1/1.1.5 with '---III'
        let coord12 = phext::to_coordinate("2.1.1/1.1.1/1.1.5");
        let update12 = phext::insert(update11.as_str(), coord12, "---III");
        assert_eq!(update12, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee---EEE\x17iii---III\x18fff\x17ggg\x19hhh");

        // extend 2.1.1/1.1.1/1.2.1 with '---FFF'
        let coord13 = phext::to_coordinate("2.1.1/1.1.1/1.2.1");
        let update13 = phext::insert(update12.as_str(), coord13, "---FFF");
        assert_eq!(update13, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee---EEE\x17iii---III\x18fff---FFF\x17ggg\x19hhh");

        // extend 2.1.1/1.1.1/1.2.2 with '---GGG'
        let coord14 = phext::to_coordinate("2.1.1/1.1.1/1.2.2");
        let update14 = phext::insert(update13.as_str(), coord14, "---GGG");
        assert_eq!(update14, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee---EEE\x17iii---III\x18fff---FFF\x17ggg---GGG\x19hhh");

        // extend 2.1.1/1.1.1/2.1.1 with '---HHH'
        let coord15 = phext::to_coordinate("2.1.1/1.1.1/2.1.1");
        let update15 = phext::insert(update14.as_str(), coord15, "---HHH");
        assert_eq!(update15, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee---EEE\x17iii---III\x18fff---FFF\x17ggg---GGG\x19hhh---HHH");

        // insert 'jjj' at 2.1.1/1.1.2/1.1.1
        let coord16 = phext::to_coordinate("2.1.1/1.1.2/1.1.1");
        let update16 = phext::insert(update15.as_str(), coord16, "jjj");
        assert_eq!(update16, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee---EEE\x17iii---III\x18fff---FFF\x17ggg---GGG\x19hhh---HHH\x1Ajjj");

        // insert 'kkk' at 2.1.1/1.2.1/1.1.1
        let coord17 = phext::to_coordinate("2.1.1/1.2.1/1.1.1");
        let update17 = phext::insert(update16.as_str(), coord17, "kkk");
        assert_eq!(update17, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee---EEE\x17iii---III\x18fff---FFF\x17ggg---GGG\x19hhh---HHH\x1Ajjj\x1Ckkk");

        // insert 'lll' at 2.1.1/2.1.1/1.1.1
        let coord18 = phext::to_coordinate("2.1.1/2.1.1/1.1.1");
        let update18 = phext::insert(update17.as_str(), coord18, "lll");
        assert_eq!(update18, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee---EEE\x17iii---III\x18fff---FFF\x17ggg---GGG\x19hhh---HHH\x1Ajjj\x1Ckkk\x1Dlll");

        // insert 'mmm' at 2.1.2/1.1.1/1.1.1
        let coord19 = phext::to_coordinate("2.1.2/1.1.1/1.1.1");
        let update19 = phext::insert(update18.as_str(), coord19, "mmm");
        assert_eq!(update19, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee---EEE\x17iii---III\x18fff---FFF\x17ggg---GGG\x19hhh---HHH\x1Ajjj\x1Ckkk\x1Dlll\x1Emmm");

        // insert 'nnn' at 2.2.1/1.1.1/1.1.1
        let coord20 = phext::to_coordinate("2.2.1/1.1.1/1.1.1");
        let update20 = phext::insert(update19.as_str(), coord20, "nnn");
        assert_eq!(update20, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee---EEE\x17iii---III\x18fff---FFF\x17ggg---GGG\x19hhh---HHH\x1Ajjj\x1Ckkk\x1Dlll\x1Emmm\x1Fnnn");

        // insert 'ooo' at 3.1.1/1.1.1/1.1.1
        let coord21 = phext::to_coordinate("3.1.1/1.1.1/1.1.1");
        let update21 = phext::insert(update20.as_str(), coord21, "ooo");
        assert_eq!(update21, "aaa---AAA\x01bbb---BBB\x17ccc---CCC\x17ddd---DDD\x17eee---EEE\x17iii---III\x18fff---FFF\x17ggg---GGG\x19hhh---HHH\x1Ajjj\x1Ckkk\x1Dlll\x1Emmm\x1Fnnn\x01ooo");
    }

    #[test]
    fn test_coordinate_based_replace() {
        // replace 'AAA' with 'aaa'
        let coord0 = phext::to_coordinate("1.1.1/1.1.1/1.1.1");
        let update0 = phext::replace("AAA\x17bbb\x18ccc\x19ddd\x1Aeee\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj", coord0, "aaa");
        assert_eq!(update0, "aaa\x17bbb\x18ccc\x19ddd\x1Aeee\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'bbb' with '222'
        let coord1 = phext::to_coordinate("1.1.1/1.1.1/1.1.2");
        let update1 = phext::replace(update0.as_str(), coord1, "222");
        assert_eq!(update1, "aaa\x17222\x18ccc\x19ddd\x1Aeee\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'ccc' with '3-'
        let coord2 = phext::to_coordinate("1.1.1/1.1.1/1.2.1");
        let update2 = phext::replace(update1.as_str(), coord2, "3-");
        assert_eq!(update2, "aaa\x17222\x183-\x19ddd\x1Aeee\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'ddd' with 'delta'
        let coord3 = phext::to_coordinate("1.1.1/1.1.1/2.1.1");
        let update3 = phext::replace(update2.as_str(), coord3, "delta");
        assert_eq!(update3, "aaa\x17222\x183-\x19delta\x1Aeee\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'eee' with 'a bridge just close enough'
        let coord4 = phext::to_coordinate("1.1.1/1.1.2/1.1.1");
        let update4 = phext::replace(update3.as_str(), coord4, "a bridge just close enough");
        assert_eq!(update4, "aaa\x17222\x183-\x19delta\x1Aa bridge just close enough\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'fff' with 'nifty'
        let coord5 = phext::to_coordinate("1.1.1/1.2.1/1.1.1");
        let update5 = phext::replace(update4.as_str(), coord5, "nifty");
        assert_eq!(update5, "aaa\x17222\x183-\x19delta\x1Aa bridge just close enough\x1Cnifty\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'ggg' with 'G8'
        let coord6 = phext::to_coordinate("1.1.1/2.1.1/1.1.1");
        let update6 = phext::replace(update5.as_str(), coord6, "G8");
        assert_eq!(update6, "aaa\x17222\x183-\x19delta\x1Aa bridge just close enough\x1Cnifty\x1DG8\x1Ehhh\x1Fiii\x01jjj");

        // replace 'hhh' with 'Hello World'
        let coord7 = phext::to_coordinate("1.1.2/1.1.1/1.1.1");
        let update7 = phext::replace(update6.as_str(), coord7, "Hello World");
        assert_eq!(update7, "aaa\x17222\x183-\x19delta\x1Aa bridge just close enough\x1Cnifty\x1DG8\x1EHello World\x1Fiii\x01jjj");

        // replace 'iii' with '_o_'
        let coord8 = phext::to_coordinate("1.2.1/1.1.1/1.1.1");
        let update8 = phext::replace(update7.as_str(), coord8, "_o_");
        assert_eq!(update8, "aaa\x17222\x183-\x19delta\x1Aa bridge just close enough\x1Cnifty\x1DG8\x1EHello World\x1F_o_\x01jjj");

        // replace 'jjj' with '/win'
        let coord9: phext::Coordinate = phext::to_coordinate("2.1.1/1.1.1/1.1.1");
        let update9 = phext::replace(update8.as_str(), coord9, "/win");
        assert_eq!(update9, "aaa\x17222\x183-\x19delta\x1Aa bridge just close enough\x1Cnifty\x1DG8\x1EHello World\x1F_o_\x01/win");
    }

    #[test]
    fn test_coordinate_based_remove() {
        // replace 'aaa' with ''
        let coord1 = phext::to_coordinate("1.1.1/1.1.1/1.1.1");
        let update1 = phext::remove("aaa\x17bbb\x18ccc\x19ddd\x1Aeee\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj", coord1);
        assert_eq!(update1, "\x17bbb\x18ccc\x19ddd\x1Aeee\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'bbb' with ''
        let coord2 = phext::to_coordinate("1.1.1/1.1.1/1.1.2");
        let update2 = phext::remove(update1.as_str(), coord2);
        assert_eq!(update2, "\x17\x18ccc\x19ddd\x1Aeee\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'ccc' with ''
        let coord3 = phext::to_coordinate("1.1.1/1.1.1/1.2.1");
        let update3 = phext::remove(update2.as_str(), coord3);
        assert_eq!(update3, "\x17\x18\x19ddd\x1Aeee\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'ddd' with ''
        let coord4 = phext::to_coordinate("1.1.1/1.1.1/2.1.1");
        let update4 = phext::remove(update3.as_str(), coord4);
        assert_eq!(update4, "\x17\x18\x19\x1Aeee\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'eee' with ''
        let coord5 = phext::to_coordinate("1.1.1/1.1.2/1.1.1");
        let update5 = phext::remove(update4.as_str(), coord5);
        assert_eq!(update5, "\x17\x18\x19\x1A\x1Cfff\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'fff' with ''
        let coord6 = phext::to_coordinate("1.1.1/1.2.1/1.1.1");
        let update6 = phext::remove(update5.as_str(), coord6);
        assert_eq!(update6, "\x17\x18\x19\x1A\x1C\x1Dggg\x1Ehhh\x1Fiii\x01jjj");

        // replace 'ggg' with ''
        let coord7 = phext::to_coordinate("1.1.1/2.1.1/1.1.1");
        let update7 = phext::remove(update6.as_str(), coord7);
        assert_eq!(update7, "\x17\x18\x19\x1A\x1C\x1D\x1Ehhh\x1Fiii\x01jjj");

        // replace 'hhh' with ''
        let coord8 = phext::to_coordinate("1.1.2/1.1.1/1.1.1");
        let update8 = phext::remove(update7.as_str(), coord8);
        assert_eq!(update8, "\x17\x18\x19\x1A\x1C\x1D\x1E\x1Fiii\x01jjj");

        // replace 'iii' with ''
        let coord9 = phext::to_coordinate("1.2.1/1.1.1/1.1.1");
        let update9 = phext::remove(update8.as_str(), coord9);
        assert_eq!(update9, "\x17\x18\x19\x1A\x1C\x1D\x1E\x1F\x01jjj");

        // replace 'jjj' with ''
        let coord10 = phext::to_coordinate("2.1.1/1.1.1/1.1.1");
        let update10 = phext::remove(update9.as_str(), coord10);
        assert_eq!(update10, "\x17\x18\x19\x1A\x1C\x1D\x1E\x1F\x01");
    }

    #[test]
    fn test_range_based_replace() {
        let doc1 = "Before\x19text to be replaced\x1Calso this\x1Dand this\x17After";
        let range1 = phext::Range { start: phext::to_coordinate("1.1.1/1.1.1/2.1.1"),
                            end: phext::to_coordinate("1.1.1/2.1.1/1.1.1") };
        let update1 = phext::range_replace(doc1, range1, "");
        assert_eq!(update1, "Before\x19\x17After");

        let doc2 = "Before\x01Library two\x01Library three\x01Library four";
        let range2 = phext::Range { start: phext::to_coordinate("2.1.1/1.1.1/1.1.1"),
                            end: phext::to_coordinate("3.1.1/1.1.1/1.1.1") };

        let update2 = phext::range_replace(doc2, range2, "");
        assert_eq!(update2, "Before\x01\x01Library four");
    }

    #[test]
    fn test_api_write() {
        // todo: figure out how to invoke rocket from unit tests
    }
}