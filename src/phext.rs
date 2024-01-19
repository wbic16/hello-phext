/// ----------------------------------------------------------------------------------------------------------
/// Phext Buffer Library
/// ported from: https://github.com/wbic16/libphext/blob/main/phext.h
///
/// Copyright: (c) 2024 Will Bickford
/// License: MIT
///
/// Overview
/// --------
/// Phext is composable, relational text. It is composed of layers upon layers of plain text (scrolls). All
/// text in a phext document is stored in subspace: a traditional buffer of utf8 text with a terminating
/// null byte. Subspace enables you to orient yourself within petabyte volumes of text with an age-old
/// mechanism: dead reckoning.
///
/// We've been using this process to keep track of columns and lines in plain text since the dawn of the
/// information age. Phext extends dead reckoning from 2D to 11D in a natural way. Please refer to this
/// article for details: https://phext.io/api.php?seed=raap&token=research&coordinate=1.1.1/1.1.1/1.1.1.
///
/// Coordinate Systems as Points
/// ----------------------------
/// To understand how truly huge a phext coordinate space is: try imagining a 3D coordinate system that
/// has been compressed fractally into a point. That's phext in a nutshell. We will make use of a series
/// of delimiters of unusual size (DOUS) to make sense of this address space.
///
/// Encoding
/// --------
/// Traditionally, text files have only contained one document type. This severely limits our ability
/// to represent arbitrary ideas and data. Phext splits file types to only have meaning within the context
/// of a scroll of text. This allows us to embed entire file systems and networks of knowledge within
/// one phext buffer. We achieve this by re-purposing historic ASCII control codes that have fallen out
/// of common use.
///
/// We've shortened these dimension names to two-letter acronyms in the table below to ensure it fits.
/// * CL = Column (1D)
/// * LN = Line (2D)
/// * SC = Scroll (3D)
/// * SN = Section (4D)
/// * CH = Chapter (5D)
/// * BK = Book (6D)
/// * VM = Volume (7D)
/// * CN = Collection (8D)
/// * SR = Series (9D)
/// * SF = Shelf (10D)
/// * LB = Library (11D)
///
/// delimiter         value     CL   LN   SC   SN   CH   BK   VM   CN   SR   SF   LB
/// ---------         -----     --   --   --   --   --   --   --   --   --   --   --
/// character         implicit  +1
/// line break        0x0A      =1   +1
/// scroll break      0x17      =1   =1   +1
/// section break     0x18      =1   =1   =1   +1
/// chapter break     0x19      =1   =1   =1   =1   +1
/// book break        0x1a      =1   =1   =1   =1   =1   +1
/// volume break      0x1c      =1   =1   =1   =1   =1   =1   +1
/// collection break  0x1d      =1   =1   =1   =1   =1   =1   =1   +1
/// series break      0x1e      =1   =1   =1   =1   =1   =1   =1   =1   +1
/// shelf break       0x1f      =1   =1   =1   =1   =1   =1   =1   =1   =1   +1
/// library break     0x01      =1   =1   =1   =1   =1   =1   =1   =1   =1   =1   +1
///
/// History Fork
/// ------------
/// Phext files are a natural fork of plain text. They add hierarchy
/// via a system of increasingly-larger dimension breaks. These breaks start
/// with normal line breaks (2D) and proceed up to library breaks (11D).
///
/// We've annotated the ascii control codes from 0x01 to 0x1f below. Phext
/// has made an effort to remain compatible with a broad swath of software.
/// It is important to note, however, that phext is a fork in the road -
/// ASCII has character codes that have fallen out of use. We've made them
/// useful again.
/// ----------------------------------------------------------------------------------------------------------

//use std::error::Error;
//use csv::ByteRecord;
//use serde::Deserialize;
//use std::default;

/// ----------------------------------------------------------------------------------------------------------
/// phext constants
/// ----------------------------------------------------------------------------------------------------------
pub const COORDINATE_MINIMUM: u8 = 1;    // human numbering - we start at 1, not 0
pub const COORDINATE_MAXIMUM: u8 = 100;  // 2 KB pages x 100^9 = 2 million petabytes
pub const LIBRARY_BREAK: u8 = 0x01;      // 11th dimension - replaces start of header
//pub const MORE_COWBELL: u8 = 0x07;     // i've got a fever, and the only prescription...is more cowbell!
//pub const LINE_BREAK:u8 = 0x0a;        // same as plain text \o/
pub const SCROLL_BREAK: u8 = 0x17;       // 3D Break - replaces End Transmission Block
pub const SECTION_BREAK: u8 = 0x18;      // 4D Break - replaces Cancel Block
pub const CHAPTER_BREAK: u8 = 0x19;      // 5D Break - replaces End of Tape
pub const BOOK_BREAK: u8 = 0x1a;         // 6D Break - replaces Substitute
pub const VOLUME_BREAK: u8 = 0x1c;       // 7D Break - replaces file separator
pub const COLLECTION_BREAK: u8 = 0x1d;   // 8D Break - replaces group separator
pub const SERIES_BREAK: u8 = 0x1e;       // 9D Break - replaces record separator
pub const SHELF_BREAK: u8 = 0x1f;        // 10D Break - replaces unit separator

pub const ADDRESS_MICRO_BREAK: u8 = b'.'; // delimiter for micro-coordinates
pub const ADDRESS_MACRO_BREAK: u8 = b'/'; // delimiter for macro-coordinates

/// ----------------------------------------------------------------------------------------------------------
/// backwards compatibility
/// -----------------------
/// phext retains backwards compatibility for a wide portion of ascii and utf8 documents. below is a summary
/// of character codes that have been retained for future growth and/or backwards-compatibility.
/// ----------------------------------------------------------------------------------------------------------
/// not widely used
/// ---------------
/// these are most useful for transmission protocols, but since http won that war, they have fallen out of
/// common use. we could evaluate some of the other characters that were kicked out above relative to these.
///
/// * start of text = 0x02
/// * end of text = 0x03
/// * end of transmission = 0x04
/// * enquery = 0x05
/// * ack = 0x06
/// * nak = 0x15
/// * syn = 0x16
/// * escape = 0x1b
///
/// actively used
/// -------------
/// * backspace = 0x08: most editors don't record backspaces, but this seems useful
/// * tab = 0x09: i had the opportunity to end the tabs vs spaces war, and chose peace
/// * vertical tab = 0x0b: seems useful
/// * form feed = 0x0c: dot matrix printers!?
/// * carriage return = 0x0d: needed for windows compatibility
/// * shift out = 0x0e: seems useful
/// * shift in = 0x0f: seems useful
/// * data link escape = 0x10: assuming printers need these
/// * device control 1 = 0x11: assuming printers need these
/// * device control 2 = 0x12: assuming printers need these
/// * device control 3 = 0x13: assuming printers need these
/// * device control 4 = 0x14: assuming printers need these
/// ----------------------------------------------------------------------------------------------------------

/// ----------------------------------------------------------------------------------------------------------
/// @struct ZCoordinate
///
/// The large-scale Z arm of a phext coordinate (see `Coordinate` below)
/// ----------------------------------------------------------------------------------------------------------
#[derive(PartialEq, Copy, Clone)]
pub struct ZCoordinate {
  library: u8,
  shelf: u8,
  series: u8
}

/// ----------------------------------------------------------------------------------------------------------
/// @struct YCoordinate
///
/// The large-scale Y arm of a phext coordinate (see `Coordinate` below)
/// ----------------------------------------------------------------------------------------------------------
#[derive(PartialEq, Copy, Clone)]
pub struct YCoordinate {
  collection: u8,
  volume: u8,
  book: u8
}

/// ----------------------------------------------------------------------------------------------------------
/// @struct XCoordinate
///
/// The large-scale X arm of a phext coordinate (see `Coordinate` below)
/// ----------------------------------------------------------------------------------------------------------
#[derive(PartialEq, Copy, Clone)]
pub struct XCoordinate {
  chapter: u8,
  section: u8,
  scroll: u8
}

/// ----------------------------------------------------------------------------------------------------------
/// @struct Coordinate
///
/// provides access to a default-initialized coordinate at 1.1.1/1.1.1/1.1.1
///
/// phext coordinates are formed along a 9-dimensional hierarchy with three main arms
/// of the form z3.z2.z1/y3.y2.y1.x3.x2.x1 where:
///
/// Z - this arm contains the library (z3), shelf (z2), and series (z1) dimensions
/// Y - this arm contains the collection (y3), volume (y2), and book (y1) dimensions
/// X - this arm contains the chapter (x3), section (x2), and scroll (x1) dimensions
/// ----------------------------------------------------------------------------------------------------------
#[derive(PartialEq, Copy, Clone)]
pub struct Coordinate {
  z: ZCoordinate,
  y: YCoordinate,
  x: XCoordinate,
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn fetch_text
///
/// retrieves the plain text string located at the given coordinates.
/// important: this can be optimized with hash tables and memo-ized parsing - for now let's keep it simple
/// see my C# implementation in https://github.com/wbic16/terse-editor if you want to do that
///
/// @param phext  the raw phext buffer to search
/// @param coord  coordinate to select the scroll from
/// ----------------------------------------------------------------------------------------------------------
pub fn fetch_text(phext: &str, target: Coordinate) -> String {
  let mut walker: Coordinate = default_coordinate();
  let mut subspace_index = 0 as usize;
  let mut stage:u8 = 0;
  let mut start = 0 as usize;
  let mut end = 0 as usize;
  let bytes = phext.as_bytes();
  let mut vec:Vec<u8> = Vec::new();

  for ptr in bytes {
    let next = *ptr;
    vec.push(next);

    if next == SCROLL_BREAK {
      walker.scroll_break();
      if stage == 1 { stage = 2; }
      subspace_index += 1;
      continue;
    }

    if next == SECTION_BREAK {
      walker.section_break();
      if stage == 1 { stage = 2; }
      subspace_index += 1;
      continue;
    }

    if next == CHAPTER_BREAK {
      walker.chapter_break();
      if stage == 1 { stage = 2; }
      subspace_index += 1;
      continue;
    }

    if next == BOOK_BREAK {
      walker.book_break();
      if stage == 1 { stage = 2; }
      subspace_index += 1;
      continue;
    }

    if next == VOLUME_BREAK {
      walker.volume_break();
      if stage == 1 { stage = 2; }
      subspace_index += 1;
      continue;
    }

    if next == COLLECTION_BREAK {
      walker.collection_break();
      if stage == 1 { stage = 2; }
      subspace_index += 1;
      continue;
    }

    if next == SERIES_BREAK {
      walker.series_break();
      if stage == 1 { stage = 2; }
      subspace_index += 1;
      continue;
    }

    if next == SHELF_BREAK {
      walker.shelf_break();
      if stage == 1 { stage = 2; }
      subspace_index += 1;
      continue;
    }

    if next == LIBRARY_BREAK {
      walker.library_break();
      if stage == 1 { stage = 2; }
      subspace_index += 1;
      continue;
    }

    if stage == 0 && target == walker {
      start = subspace_index;
      stage = 1;
    }

    subspace_index += 1;
  }

  if stage == 1 {
    end = (vec.len() - 1) as usize;
  }

  if end > start
  {
    let temp = vec.into_iter().skip(start).take(end - start).collect();
    return String::from_utf8(temp).expect("Invalid UTF-8");
  }

  return "".to_owned();
}

pub fn default_coordinate() -> Coordinate {
  let coord = Coordinate {
    z: ZCoordinate {
      library: 1,
      shelf: 1,
      series: 1
    },
    y: YCoordinate {
      collection: 1,
      volume: 1,
      book: 1
    },
    x: XCoordinate {
      chapter: 1,
      section: 1,
      scroll: 1
    }
  };
  return coord;
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn to_coordinate
///
/// translates a phext string to a strongly-typed address
///
/// @param address  text to parse
/// ----------------------------------------------------------------------------------------------------------
pub fn to_coordinate(address: &str) -> Coordinate {
  let mut result: Coordinate = default_coordinate();

  let mut index: u8 = 0;
  let mut value:u32 = 0;
  let exp:u32 = 10;

  for next in address.as_bytes() {
    let byte = *next;

    if byte == ADDRESS_MICRO_BREAK || byte == ADDRESS_MACRO_BREAK {
      value = 0;
      index += 1;

      match index {
        1 => {result.z.library = value as u8},
        2 => {result.z.shelf = value as u8},
        3 => {result.z.series = value as u8},
        4 => {result.y.collection = value as u8},
        5 => {result.y.volume = value as u8},
        6 => {result.y.book = value as u8},
        7 => {result.x.chapter = value as u8},
        8 => {result.x.section = value as u8},
        _ => {}
      }
    }

    if byte >= 0x30 && byte <= 0x39
    {
      value = exp * value + ((byte - 0x30) as u32);
    }
  }

  result.x.scroll = value as u8;

  return result;
}

fn validate_dimension_index(index: u8) -> bool {
  return index >= COORDINATE_MINIMUM && index <= COORDINATE_MAXIMUM;
}

impl Coordinate {
  /// ----------------------------------------------------------------------------------------------------------
  /// @fn validate_coordinate
  ///
  /// determines if coord points to a valid phext address
  ///
  /// @param coord: the coordinate to reset
  /// ----------------------------------------------------------------------------------------------------------
  pub fn validate_coordinate(&self) -> bool {
    let ok = validate_dimension_index(self.z.library) &&
                   validate_dimension_index(self.z.shelf) &&
                   validate_dimension_index(self.z.series) &&
                   validate_dimension_index(self.y.collection) &&
                   validate_dimension_index(self.y.volume) &&
                   validate_dimension_index(self.y.book) &&
                   validate_dimension_index(self.x.chapter) &&
                   validate_dimension_index(self.x.section) &&
                   validate_dimension_index(self.x.scroll);
    return ok;
  }

  /// ----------------------------------------------------------------------------------------------------------
  /// @fn to_string
  ///
  /// produces a quoted string for the given phext address in canonical format (z3.z2.z1/y3.y2.y1/x3.x2.x1)
  ///
  /// @param coord  the coordinate to translate
  /// ----------------------------------------------------------------------------------------------------------
  pub fn to_string(&self) -> String {
    if !self.validate_coordinate() {
      return "".to_owned();
    }
    return format!("{}.{}.{}/{}.{}.{}/{}.{}.{}",
      self.z.library, self.z.shelf, self.z.series,
      self.y.collection, self.y.volume, self.y.book,
      self.x.chapter, self.x.section, self.x.scroll);
  }

  /// ----------------------------------------------------------------------------------------------------------
  /// @fn advance_coordinate
  ///
  /// ----------------------------------------------------------------------------------------------------------
  fn advance_coordinate(index: u8) -> u8 {
    let next = index + 1;
    if next < COORDINATE_MAXIMUM {
      return next;
    }

    return index; // can't advance beyond the maximum
  }

  /// ------------------------------------------------------------------------------------------------------
  /// @fn scroll_break
  /// ------------------------------------------------------------------------------------------------------
  pub fn scroll_break(&mut self) {
    self.x.scroll = Self::advance_coordinate(self.x.scroll);
  }

  /// ------------------------------------------------------------------------------------------------------
  /// @fn section_break
  /// ------------------------------------------------------------------------------------------------------
  pub fn section_break(&mut self) {
    self.x.section = Self::advance_coordinate(self.x.section);
    self.x.scroll = 1;
  }

  /// ------------------------------------------------------------------------------------------------------
  /// @fn chapter_break
  /// ------------------------------------------------------------------------------------------------------
  pub fn chapter_break(&mut self) {
    self.x.chapter = Self::advance_coordinate(self.x.chapter);
    self.x.section = 1;
    self.x.scroll = 1;
  }

  /// ------------------------------------------------------------------------------------------------------
  /// @fn book_break
  /// ------------------------------------------------------------------------------------------------------
  pub fn book_break(&mut self) {
    self.y.book = Self::advance_coordinate(self.y.book);
    self.x.chapter = 1;
    self.x.section = 1;
    self.x.scroll = 1;
  }

  /// ------------------------------------------------------------------------------------------------------
  /// @fn volume_break
  /// ------------------------------------------------------------------------------------------------------
  pub fn volume_break(&mut self) {
    self.y.volume = Self::advance_coordinate(self.y.volume);
    self.y.book = 1;
    self.x.chapter = 1;
    self.x.section = 1;
    self.x.scroll = 1;
  }

  /// ------------------------------------------------------------------------------------------------------
  /// @fn collection_break
  /// ------------------------------------------------------------------------------------------------------
  pub fn collection_break(&mut self) {
    self.y.collection = Self::advance_coordinate(self.y.collection);
    self.y.volume = 1;
    self.y.book = 1;
    self.x.chapter = 1;
    self.x.section = 1;
    self.x.scroll = 1;
  }

  /// ------------------------------------------------------------------------------------------------------
  /// @fn series_break
  /// ------------------------------------------------------------------------------------------------------
  pub fn series_break(&mut self) {
    self.z.series = Self::advance_coordinate(self.z.series);
    self.y.collection = 1;
    self.y.volume = 1;
    self.y.book = 1;
    self.x.chapter = 1;
    self.x.section = 1;
    self.x.scroll = 1;
  }

  /// ------------------------------------------------------------------------------------------------------
  /// @fn shelf_break
  /// ------------------------------------------------------------------------------------------------------
  pub fn shelf_break(&mut self) {
    self.z.shelf = Self::advance_coordinate(self.z.shelf);
    self.z.series = 1;
    self.y.book = 1;
    self.y.collection = 1;
    self.y.volume = 1;
    self.y.book = 1;
    self.x.chapter = 1;
    self.x.section = 1;
    self.x.scroll = 1;
  }

  /// ------------------------------------------------------------------------------------------------------
  /// @fn library_break
  /// ------------------------------------------------------------------------------------------------------
  pub fn library_break(&mut self) {
    self.z.library = Self::advance_coordinate(self.z.library);
    self.z.shelf = 1;
    self.z.series = 1;
    self.y.book = 1;
    self.y.collection = 1;
    self.y.volume = 1;
    self.y.book = 1;
    self.x.chapter = 1;
    self.x.section = 1;
    self.x.scroll = 1;
  }
}