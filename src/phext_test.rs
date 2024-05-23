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
        let update2 = phext::insert(update1.as_str(), coord2, "ddd");
        assert_eq!(update2, "aaa\x01bbb\x17ccc\x17ddd\x17eee");
    }

    #[test]
    fn test_api_write() {
        // todo: figure out how to invoke rocket from unit tests
    }
}