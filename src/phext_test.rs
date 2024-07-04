#[cfg(test)]
mod tests {
    use crate::phext::{self, PositionedScroll, BOOK_BREAK, CHAPTER_BREAK, COLLECTION_BREAK, LIBRARY_BREAK, SCROLL_BREAK, SECTION_BREAK, SERIES_BREAK, SHELF_BREAK, VOLUME_BREAK};
    use std::{collections::HashMap, io::Write};

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
        assert_eq!(expected1.0, 11);
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
        let coord3 = phext::to_coordinate("2.1.1/1.1.1/1.2.1");
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
    fn test_next_scroll() {
        let doc1 = "3A\x17B2\x18C1";
        let (update1, next_start, remaining) = phext::next_scroll(doc1, phext::to_coordinate("1.1.1/1.1.1/1.1.1"));
        assert_eq!(update1.coord.to_string(), "1.1.1/1.1.1/1.1.1");        
        assert_eq!(update1.scroll, "3A");
        assert_eq!(next_start.to_string(), "1.1.1/1.1.1/1.1.2");
        assert_eq!(remaining, "B2\x18C1");
    }

    #[test]
    fn test_phokenize() {
        let doc1 = "one\x17two\x17three\x17four";
        let mut expected1: Vec<phext::PositionedScroll> = Vec::new();
        expected1.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/1.1.1/1.1.1"), scroll: "one".to_string()});
        expected1.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/1.1.1/1.1.2"), scroll: "two".to_string()});
        expected1.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/1.1.1/1.1.3"), scroll: "three".to_string()});
        expected1.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/1.1.1/1.1.4"), scroll: "four".to_string()});
        let update1: Vec<PositionedScroll> = phext::phokenize(doc1);
        assert_eq!(update1, expected1);

        let doc2 = "one\x01two\x1Fthree\x1Efour\x1Dfive\x1Csix\x1Aseven\x19eight\x18nine\x17ten";
        let mut expected2: Vec<phext::PositionedScroll> = Vec::new();
        expected2.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/1.1.1/1.1.1"), scroll: "one".to_string()});
        expected2.push(PositionedScroll{ coord: phext::to_coordinate("2.1.1/1.1.1/1.1.1"), scroll: "two".to_string()});
        expected2.push(PositionedScroll{ coord: phext::to_coordinate("2.2.1/1.1.1/1.1.1"), scroll: "three".to_string()});
        expected2.push(PositionedScroll{ coord: phext::to_coordinate("2.2.2/1.1.1/1.1.1"), scroll: "four".to_string()});
        expected2.push(PositionedScroll{ coord: phext::to_coordinate("2.2.2/2.1.1/1.1.1"), scroll: "five".to_string()});
        expected2.push(PositionedScroll{ coord: phext::to_coordinate("2.2.2/2.2.1/1.1.1"), scroll: "six".to_string()});
        expected2.push(PositionedScroll{ coord: phext::to_coordinate("2.2.2/2.2.2/1.1.1"), scroll: "seven".to_string()});
        expected2.push(PositionedScroll{ coord: phext::to_coordinate("2.2.2/2.2.2/2.1.1"), scroll: "eight".to_string()});
        expected2.push(PositionedScroll{ coord: phext::to_coordinate("2.2.2/2.2.2/2.2.1"), scroll: "nine".to_string()});
        expected2.push(PositionedScroll{ coord: phext::to_coordinate("2.2.2/2.2.2/2.2.2"), scroll: "ten".to_string()});
        let update2: Vec<PositionedScroll> = phext::phokenize(doc2);
        assert_eq!(update2, expected2);

        let doc3 = "one\x17two\x18three\x19four\x1afive\x1csix\x1dseven\x1eeight\x1fnine\x01ten";
        let mut expected3: Vec<phext::PositionedScroll> = Vec::new();
        expected3.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/1.1.1/1.1.1"), scroll: "one".to_string()});
        expected3.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/1.1.1/1.1.2"), scroll: "two".to_string()});
        expected3.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/1.1.1/1.2.1"), scroll: "three".to_string()});
        expected3.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/1.1.1/2.1.1"), scroll: "four".to_string()});
        expected3.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/1.1.2/1.1.1"), scroll: "five".to_string()});
        expected3.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/1.2.1/1.1.1"), scroll: "six".to_string()});
        expected3.push(PositionedScroll{ coord: phext::to_coordinate("1.1.1/2.1.1/1.1.1"), scroll: "seven".to_string()});
        expected3.push(PositionedScroll{ coord: phext::to_coordinate("1.1.2/1.1.1/1.1.1"), scroll: "eight".to_string()});
        expected3.push(PositionedScroll{ coord: phext::to_coordinate("1.2.1/1.1.1/1.1.1"), scroll: "nine".to_string()});
        expected3.push(PositionedScroll{ coord: phext::to_coordinate("2.1.1/1.1.1/1.1.1"), scroll: "ten".to_string()});
        let update3: Vec<PositionedScroll> = phext::phokenize(doc3);
        assert_eq!(update3, expected3);

        let doc4 = "\x1A\x1C\x1D\x1E\x1F\x01stuff here";
        let mut expected4: Vec<phext::PositionedScroll> = Vec::new();
        expected4.push(PositionedScroll{ coord: phext::to_coordinate("2.1.1/1.1.1/1.1.1"), scroll: "stuff here".to_string()});
        let update4: Vec<PositionedScroll> = phext::phokenize(doc4);
        assert_eq!(update4, expected4);
    }

    #[test]
    fn test_merge() {
        let doc_1a = "3A\x17B2";
        let doc_1b = "4C\x17D1";
        let update_1 = phext::merge(doc_1a, doc_1b);
        assert_eq!(update_1, "3A4C\x17B2D1");

        let doc_2a = "Hello \x17I've come to talk";
        let doc_2b = "Darkness, my old friend.\x17 with you again.";
        let update_2 = phext::merge(doc_2a, doc_2b);
        assert_eq!(update_2, "Hello Darkness, my old friend.\x17I've come to talk with you again.");

        let doc_3a = "One\x17Two\x18Three\x19Four";
        let doc_3b = "1\x172\x183\x194";
        let update_3 = phext::merge(doc_3a, doc_3b);
        assert_eq!(update_3, "One1\x17Two2\x18Three3\x19Four4");

        let doc_4a = "\x1A\x1C\x1D\x1E\x1F\x01stuff here";
        let doc_4b = "\x1A\x1C\x1D\x1Eprecursor here\x1F\x01and more";
        let update_4 = phext::merge(doc_4a, doc_4b);
        assert_eq!(update_4, "\x1Eprecursor here\x01stuff hereand more");

        let doc_5a = "\x01\x01 Library at 3.1.1/1.1.1/1.1.1 \x1F Shelf at 3.2.1/1.1.1/1.1.1";
        let doc_5b = "\x01\x01\x01 Library 4.1.1/1.1.1/1.1.1 \x1E Series at 4.1.2/1.1.1/1.1.1";
        let update_5 = phext::merge(doc_5a, doc_5b);
        assert_eq!(update_5, "\x01\x01 Library at 3.1.1/1.1.1/1.1.1 \x1F Shelf at 3.2.1/1.1.1/1.1.1\x01 Library 4.1.1/1.1.1/1.1.1 \x1E Series at 4.1.2/1.1.1/1.1.1");

        let doc_6a = "\x1D Collection at 1.1.1/2.1.1/1.1.1\x1C Volume at 1.1.1/2.2.1/1.1.1";
        let doc_6b = "\x1D\x1D Collection at 1.1.1/3.1.1/1.1.1\x1C Volume at 1.1.1/3.2.1/1.1.1";
        let update_6 = phext::merge(doc_6a, doc_6b);
        assert_eq!(update_6, "\x1D Collection at 1.1.1/2.1.1/1.1.1\x1C Volume at 1.1.1/2.2.1/1.1.1\x1D Collection at 1.1.1/3.1.1/1.1.1\x1C Volume at 1.1.1/3.2.1/1.1.1");

        let doc_7a = "\x1ABook #2 Part 1\x1ABook #3 Part 1";
        let doc_7b = "\x1A + Part II\x1A + Part Deux";
        let update_7 = phext::merge(doc_7a, doc_7b);
        assert_eq!(update_7, "\x1ABook #2 Part 1 + Part II\x1ABook #3 Part 1 + Part Deux");
    
        let doc8a = "AA\x01BB\x01CC";
        let doc8b = "__\x01__\x01__";
        let update8 = phext::merge(doc8a, doc8b);
        assert_eq!(update8, "AA__\x01BB__\x01CC__");
    }

    #[test]
    fn test_subtract() {
        let doc1a = "Here's scroll one.\x17Scroll two.";
        let doc1b = "Just content at the first scroll";
        let update1 = phext::subtract(doc1a, doc1b);
        assert_eq!(update1, "\x17Scroll two.");
    }

    #[test]
    fn test_normalize() {
        let doc1 = "\x17Scroll two\x18\x18\x18\x18";
        let update1 = phext::normalize(doc1);
        assert_eq!(update1, "\x17Scroll two");
    }

    #[test]
    fn test_expand() {
        let doc1 = "nothing but line breaks\x0Ato test expansion to scrolls\x0Aline 3";
        let update1 = phext::expand(doc1);
        assert_eq!(update1, "nothing but line breaks\x17to test expansion to scrolls\x17line 3");

        let update2 = phext::expand(update1.as_str());
        assert_eq!(update2, "nothing but line breaks\x18to test expansion to scrolls\x18line 3");

        let update3 = phext::expand(update2.as_str());
        assert_eq!(update3, "nothing but line breaks\x19to test expansion to scrolls\x19line 3");

        let update4 = phext::expand(update3.as_str());
        assert_eq!(update4, "nothing but line breaks\x1Ato test expansion to scrolls\x1Aline 3");

        let update5 = phext::expand(update4.as_str());
        assert_eq!(update5, "nothing but line breaks\x1Cto test expansion to scrolls\x1Cline 3");

        let update6 = phext::expand(update5.as_str());
        assert_eq!(update6, "nothing but line breaks\x1Dto test expansion to scrolls\x1Dline 3");

        let update7 = phext::expand(update6.as_str());
        assert_eq!(update7, "nothing but line breaks\x1Eto test expansion to scrolls\x1Eline 3");

        let update8 = phext::expand(update7.as_str());
        assert_eq!(update8, "nothing but line breaks\x1Fto test expansion to scrolls\x1Fline 3");

        let update9 = phext::expand(update8.as_str());
        assert_eq!(update9, "nothing but line breaks\x01to test expansion to scrolls\x01line 3");

        let update10 = phext::expand(update9.as_str());
        assert_eq!(update10, "nothing but line breaks\x01to test expansion to scrolls\x01line 3");
    }

    #[test]
    fn test_contract() {
        let doc1 = "A more complex example than expand\x01----\x1F++++\x1E____\x1Doooo\x1C====\x1Azzzz\x19gggg\x18....\x17qqqq";
        let update1 = phext::contract(doc1);
        assert_eq!(update1, "A more complex example than expand\x1F----\x1E++++\x1D____\x1Coooo\x1A====\x19zzzz\x18gggg\x17....\x0Aqqqq");

        let update2 = phext::contract(update1.as_str());
        assert_eq!(update2, "A more complex example than expand\x1E----\x1D++++\x1C____\x1Aoooo\x19====\x18zzzz\x17gggg\x0A....\x0Aqqqq");
    }

    #[test]
    fn test_swap() {
        let doc1a = "text in scroll #1\x17scroll #2";
        let doc1b = "first\x17second";
        let coord1 = phext::to_coordinate("1.1.1/1.1.1/1.1.1");
        let update1 = phext::swap(coord1, doc1a, doc1b);
        assert_eq!(update1, "first\x17scroll #2");

        let doc2a = "before\x18section two\x18section three\x18section four";
        let doc2b = "lalala\x18replaced!!!\x18ignored\x19ignored\x17ignored";
        let coord2 = phext::to_coordinate("1.1.1/1.1.1/1.2.1");
        let update2 = phext::swap(coord2, doc2a, doc2b);
        assert_eq!(update2, "before\x18replaced!!!\x18section three\x18section four");

        let doc3a = "one\x19two\x19three\x19four";
        let doc3b = "111\x192\x193\x194";
        let coord3 = phext::to_coordinate("1.1.1/1.1.1/3.1.1");
        let update3 = phext::swap(coord3, doc3a, doc3b);
        assert_eq!(update3, "one\x19two\x193\x19four");

        let doc4a = "alpha\x1abeta\x1agamma\x1adelta";
        let doc4b = "123\x1a456\x1a789\x1a842";
        let coord4 = phext::to_coordinate("1.1.1/1.1.4/1.1.1");
        let update4 = phext::swap(coord4, doc4a, doc4b);
        assert_eq!(update4, "alpha\x1abeta\x1agamma\x1a842");

        let doc5a = "alpha\x1cbeta\x1csuper\x1cduty\x1cextra";
        let doc5b = "\x1c\x1c\x1c\x1c new text here ";
        let coord5 = phext::to_coordinate("1.1.1/1.5.1/1.1.1");
        let update5 = phext::swap(coord5, doc5a, doc5b);
        assert_eq!(update5, "alpha\x1cbeta\x1csuper\x1cduty\x1c new text here ");

        let doc6a = "stuff\x1d and things \x1d and more \x1d goes here and there";
        let doc6b = "111 \x1d 222 \x1d 333 \x1d 444 \x1d 555";
        let coord6 = phext::to_coordinate("1.1.1/4.1.1/1.1.1");
        let update6 = phext::swap(coord6, doc6a, doc6b);
        assert_eq!(update6, "stuff\x1d and things \x1d and more \x1d 444 ");

        let doc7a = "aaa \x1e bbb \x1e ccc \x1e ddd \x1e eee";
        let doc7b = "A\x1eB\x1eC\x1eD\x1eE\x1eF\x1eG";
        let coord7 = phext::to_coordinate("1.1.3/1.1.1/1.1.1");
        let update7 = phext::swap(coord7, doc7a, doc7b);
        assert_eq!(update7, "aaa \x1e bbb \x1eC\x1e ddd \x1e eee");

        let doc8a = "aaaaa \x1f bbbb \x1f ccc \x1f dddddd \x1f eeeeeeee \x1f ff ";
        let doc8b = "\x1f\x1f\x1f\x1f___\x1f\x1f\x1f\x1f\x1f\x1f";
        let coord8 = phext::to_coordinate("1.5.1/1.1.1/1.1.1");
        let update8 = phext::swap(coord8, doc8a, doc8b);
        assert_eq!(update8, "aaaaa \x1f bbbb \x1f ccc \x1f dddddd \x1f___\x1f ff ");

        let doc9a = "lib 1 \x01 lib 2 \x01 lib 3 \x01 lib 4 \x01 lib 5";
        let doc9b = "zzz \x01 yyy \x01 xxx \x01 www \x01 vvv \x01 uuu \x01";
        let coord9 = phext::to_coordinate("2.1.1/1.1.1/1.1.1");
        let update9 = phext::swap(coord9, doc9a, doc9b);
        assert_eq!(update9, "lib 1 \x01 yyy \x01 lib 3 \x01 lib 4 \x01 lib 5");
    }

    #[test]
    fn test_fs_read_write() {
        let filename = "unit-test.phext";
        let file = std::fs::File::create(&filename);
        let required = "Unable to locate ".to_owned() + &filename;
        let initial = "a simple phext doc with three scrolls\x17we just want to verify\x17that all of our breaks are making it through rust's fs layer.\x18section 2\x19chapter 2\x1Abook 2\x1Cvolume 2\x1Dcollection 2\x1Eseries 2\x1Fshelf 2\x01library 2";
        let _result = file.expect(&required).write_all(initial.as_bytes());
        let prior = std::fs::read_to_string(filename).expect("Unable to open phext");

        assert_eq!(prior, initial);
        let coordinate = "2.1.1/1.1.1/1.1.1";
        let message = phext::replace(prior.as_str(), phext::to_coordinate(coordinate), "still lib 2");
        assert_eq!(message, "a simple phext doc with three scrolls\x17we just want to verify\x17that all of our breaks are making it through rust's fs layer.\x18section 2\x19chapter 2\x1Abook 2\x1Cvolume 2\x1Dcollection 2\x1Eseries 2\x1Fshelf 2\x01still lib 2");
    }

    #[test]
    fn test_replace_create() {
        let initial = "A\x17B\x17C\x18D\x19E\x1AF\x1CG\x1DH\x1EI\x1FJ\x01K";
        let coordinate = "3.1.1/1.1.1/1.1.1";
        let message = phext::replace(initial, phext::to_coordinate(coordinate), "L");
        assert_eq!(message, "A\x17B\x17C\x18D\x19E\x1AF\x1CG\x1DH\x1EI\x1FJ\x01K\x01L");
    }
}