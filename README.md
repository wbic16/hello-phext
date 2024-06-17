# hello-phext

This repository demonstrates how to work with phext from a rust context. Phext is 11 dimensional plain hypertext.

## Build

1. Clone this repo
2. Install Rust
3. Run `cargo build`

## Test

1. Complete the build steps above
2. Run `cargo test`

## Examples

After completing the build and test commands, you should be able to try out the example, pack, and unpack commands.

### Demonstrate how to inspect a phext
Run `cargo run help`

### Compile all local text files into a phext
Run `cargo run pack test.phext`

### Extract all nodes from a phext document into your local directory
Not yet implemented

### Unit Tests

* coordinate_parsing: Verifies that string -> coordinate -> string produces the same result
* scrolls: Verifies that SCROLL_BREAK reliably splits 3 scrolls
* sections: Verifies that SECTION_BREAK reliably splits 3 sections
* chapters: Verifies that CHAPTER_BREAK reliably splits 3 chapters
* books: Verifies that BOOK_BREAK reliably splits 3 books
* volumes: Verifies that VOLUME_BREAK reliably splits 3 volumes
* collections: Verifies that COLLECTION_BREAK reliably splits 3 collections
* series: Verifies that SERIES_BREAK reliably splits 3 series
* shelves: Verifies that SHELF_BREAK reliably splits 3 shelves
* libraries: Verifies that LIBRARY_BREAK reliably splits 3 libraries
* coordinates_invalid: tests for invalid coordinate detection
* coordinates_valid: ensures that a realistic coordinate is valid
* test_url_encoding: tests for alternate url format with semicolons
* realistic_parse: Verifies that a coordinate with many delimiters parses correctly
* dead_reckoning: Verifies that we can accurately calculate coordinates on existing phext documents
* line_break: Proves that we're using ASCII line breaks
* test_more_cowbell: Ensures that you've got more cowbell!
* coordinate_based_insert: Verifies that random insertion by phext coordinate works
* coordinate_based_replace: Verifies that random replacement by phext coordinate works
* coordinate_based_remove: Verifies that random scroll removal by phext coordinate works
* range_based_replace: Verifies that a range of phext coordinates can be used to replace text
* next_scroll: verifies that we can tokenize subspace by scroll
* expand: verifies that delimiters can be grown larger by 1 dimension
* contract: verifies that delimiters can be shrunk by 1 dimension
* WIP: merge: verifies that two phext documents can be zipper-merged
* PENDING: intersection: verifies that content from shared coordinates can be merged between two docs
* PENDING: subtract: verifies that we can prune all of the coordinates from a second phext document
* PENDING: normalize: verifies that empty scrolls are pruned from the given phext document
* PENDING: swap: verifies that the content at the given coordinates can be swapped between two phext archives
* test_api_write
