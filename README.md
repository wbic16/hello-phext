# hello-phext

This Rust project demonstrates how to work with Phext (11-dimensional plain hypertext). For more information about the phext format, head over to https://phext.io.

## Elevator Pitch

Phext is hierarchical digital memory. It enables seamless knowledge transfer between humans and computers. Let's learn how to think at planet-scale. :)

## Hello, Phext

This project is an amalgamation of a Rust-powered Phext backend (phext.rs) and a Rocket-based API (main.rs). The functionality defined in the Phext backend provides all of the database/logic needed for hosting phext documents. The Rocket-based API is a wrapper around that functionality.

## Phext Motivation

In the 1980s, computers could write 25 KB/sec to a floppy disk. In the 2020s, it became possible to write 2 GB/sec to an SSD. This changed the definition of a "small" file. Unfortunately, most of our file abstractions (especially on Windows) have not scaled to take advantage of these performance gains. For the most part, this isn't much of a problem: humans are still rate-limited at 300 bps using keyboards. At some point in the next 25 years, however, we will have high-bandwidth brain interconnects - at which point we will need a high-bandwidth multi-dimensional text format: phext!

The introduction of Large Language Models (LLMs) has accelerated our transition to this future. You can use phext to interact with agents and groups of humans at scale - think of visualizing 9 billion computer screens at once. Phext is like being given a coordinate system of coordinate systems, allowing you to walk the latent space of any problem space efficiently.

## Phext Coordinate Formats

* Canonical Format: Orders coordinates to avoid the need for labels
  * example: z3.z2.z1/y3.y2.y1/x3.x2.x1
  * z3 - Library (LB)
  * z2 - Shelf (SF)
  * z1 - Series (SR)
  * y3 - Collection (CN)
  * y2 - Volume (VM)
  * y1 - Book (BK)
  * x3 - Chapter (CH)
  * x2 - Section (SN)
  * x1 - Scroll (SC)
* URL Format: the same as the canonical format, but with semi-colons instead of slashes
  * this allows us to use coordinates in routes
  * example: z3.z2.z1;y3.y2.y1;x3.x2.x1

## Build

1. Clone this repo
2. Install Rust
3. Run `cargo build`

## Test

1. Complete the build steps above
2. Run `cargo test`

## Run

1. After building and testing the project, start the rocket server.
2. Run `cargo run`

### API Routes (User Level)

* Basic View: `/api/v1/index/{world}/{coordinate}`
  * world: the filename of the .phext archive to load
  * coordinate: the phext coordinate in URL form (z3.z2.z1;y3.y2.y1;x3.x2.x1)
* Save Scroll: `/api/v1/save/{world}/{coordinate}`
  * Runs `phext::replace` for the given coordinate within "world.phext"
* Normalize Phext: `/api/v1/normalize/{world}`
  * Accepts the posted scroll content as "world.phext" *and* normalizes the output
* Contract Phext: `/api/v1/contract/{world}`
  * Accepts the posted scroll content as "world.phext" *and* adjusts all phext delimiters down by 1 dimension
* Expand Phext: `/api/v1/expand/{world}`
  * Accepts the posted scroll content as "world.phext" *and* adjusts all phext delimiters up by 1 dimension
* Liquid Visualizer: `/api/v1/liquid/{world}/{coordinate}`
  * Provides a tile grid of 9,801 scrolls in an 11x11 group of 9x9 scrolls
  * The background color is rendered from the rgb value computed by R=LB/99 G=SF/99 B=SR/99
  * The tile background color is rendered from the rgb value computed by R=CN/99 G=VM/99 B=BK/99

### Low-Level (SQL-ish) Routes

Note: You can omit the coordinate part of the route to work in subspace directly.

* Select: `/api/v1/select/{world}/{coordinate}`
  * fetches the scroll from "world.phext" at the given coordinate
* Insert: `/api/v1/insert/{world}/{coordinate}`
  * Appends text at the given coordinates (expanding that scroll)
* Update: `/api/v1/update/{world}/{coordinate}`
  * Replaces the content at the given coordinates (used by Save)
* Delete: `/api/v1/delete/{world}/{coordinate}`
  * zeroizes the scroll at the given coordinates
  * zeroizes the entire contents of "world.phext" if no coordinate is given
* Select Phext: `/api/v1/select/{world}`
  * fetches "world.phext"
* Insert Phext: `/api/v1/insert/{world}`
  * replaces the given phext doc
* Update Phext: `/api/v1/update/{world}`
  * Replaces the entire contents of the given phext doc
* Delete Phext: `/api/v1/delete/{world}`
  * zeroizes the entire contents of "world.phext"
* Dummy Route: `/api/{world}/catchall`
  * a dummy route for collecting methods not used elsewhere (yet)

### Phext Basics

* test_more_cowbell: Ensures that you've got more cowbell!
* line_break: Proves that we're using ASCII line breaks
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
* realistic_parse: Verifies that a coordinate with many delimiters parses correctly
* dead_reckoning: Verifies that we can accurately calculate coordinates on existing phext documents

### API Tests

* next_scroll: verifies that we can tokenize subspace by scroll
* phokenize: verifies that we can build subspace phokens (phext tokens)
* test_url_encoding: tests for alternate url format with semicolons
* coordinate_based_insert: Verifies that random insertion by phext coordinate works
* coordinate_based_replace: Verifies that random replacement by phext coordinate works
* coordinate_based_remove: Verifies that random scroll removal by phext coordinate works
* range_based_replace: Verifies that a range of phext coordinates can be used to replace text
* expand: verifies that delimiters can be grown larger by 1 dimension
* contract: verifies that delimiters can be shrunk by 1 dimension
* merge: verifies that two phext documents can be zipper-merged (intersection)
* subtract: verifies that we can prune all of the coordinates from a second phext document
* normalize: verifies that empty scrolls are pruned from the given phext document
