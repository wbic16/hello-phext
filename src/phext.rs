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
///
///   Dimension  Designation  Description
///   ---------  -----------  ----------- 
///   1          CL           Column
///   2          LN           Line
///   3          SC           Scroll
///   4          SN           Section
///   5          CH           Chapter
///   6          BK           Book
///   7          VM           Volume
///   8          CN           Collection
///   9          SR           Series
///   10         SF           Shelf
///   11         LB           Library
///
///   delimiter         value     CL   LN   SC   SN   CH   BK   VM   CN   SR   SF   LB
///   ---------         -----     --   --   --   --   --   --   --   --   --   --   --
///   character         implicit  +1
///   line break        0x0A      =1   +1
///   scroll break      0x17      =1   =1   +1
///   section break     0x18      =1   =1   =1   +1
///   chapter break     0x19      =1   =1   =1   =1   +1
///   book break        0x1A      =1   =1   =1   =1   =1   +1
///   volume break      0x1C      =1   =1   =1   =1   =1   =1   +1
///   collection break  0x1D      =1   =1   =1   =1   =1   =1   =1   +1
///   series break      0x1E      =1   =1   =1   =1   =1   =1   =1   =1   +1
///   shelf break       0x1F      =1   =1   =1   =1   =1   =1   =1   =1   =1   +1
///   library break     0x01      =1   =1   =1   =1   =1   =1   =1   =1   =1   =1   +1
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

/// ----------------------------------------------------------------------------------------------------------
/// phext constants
/// ----------------------------------------------------------------------------------------------------------
pub const COORDINATE_MINIMUM: usize = 1;   // human numbering - we start at 1, not 0
pub const COORDINATE_MAXIMUM: usize = 100; // 2 KB pages x 100^9 = 2 million petabytes
pub const LIBRARY_BREAK: char = '\x01';    // 11th dimension - replaces start of header
pub const MORE_COWBELL: char = '\x07';     // i've got a fever, and the only prescription...is more cowbell!
pub const LINE_BREAK: char = '\x0A';       // same as plain text \o/
pub const SCROLL_BREAK: char = '\x17';     // 3D Break - replaces End Transmission Block
pub const SECTION_BREAK: char = '\x18';    // 4D Break - replaces Cancel Block
pub const CHAPTER_BREAK: char = '\x19';    // 5D Break - replaces End of Tape
pub const BOOK_BREAK: char = '\x1A';       // 6D Break - replaces Substitute
pub const VOLUME_BREAK: char = '\x1C';     // 7D Break - replaces file separator
pub const COLLECTION_BREAK: char = '\x1D'; // 8D Break - replaces group separator
pub const SERIES_BREAK: char = '\x1E';     // 9D Break - replaces record separator
pub const SHELF_BREAK: char = '\x1F';      // 10D Break - replaces unit separator

pub const ADDRESS_MICRO_BREAK: u8 = b'.'; // delimiter for micro-coordinates
pub const ADDRESS_MACRO_BREAK: u8 = b'/'; // delimiter for macro-coordinates
pub const ADDRESS_MACRO_ALT: u8 = b';';   // also allow ';' for url encoding

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
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ZCoordinate {
  pub library: usize,
  pub shelf: usize,
  pub series: usize
}
impl Default for ZCoordinate {
  fn default() -> ZCoordinate {
    ZCoordinate {
      library: 1,
      shelf: 1,
      series: 1
    }
  }
}
impl std::fmt::Display for ZCoordinate {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}.{}.{}", self.library, self.shelf, self.series);
  }
}

/// ----------------------------------------------------------------------------------------------------------
/// @struct YCoordinate
///
/// The large-scale Y arm of a phext coordinate (see `Coordinate` below)
/// ----------------------------------------------------------------------------------------------------------
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct YCoordinate {
  pub collection: usize,
  pub volume: usize,
  pub book: usize
}
impl Default for YCoordinate {
  fn default() -> YCoordinate {
    YCoordinate {
      collection: 1,
      volume: 1,
      book: 1
    }
  }
}
impl std::fmt::Display for YCoordinate {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}.{}.{}", self.collection, self.volume, self.book);
  }
}

/// ----------------------------------------------------------------------------------------------------------
/// @struct XCoordinate
///
/// The large-scale X arm of a phext coordinate (see `Coordinate` below)
/// ----------------------------------------------------------------------------------------------------------
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct XCoordinate {
  pub chapter: usize,
  pub section: usize,
  pub scroll: usize
}
impl Default for XCoordinate {
  fn default() -> XCoordinate {
    XCoordinate {
      chapter: 1,
      section: 1,
      scroll: 1
    }
  }
}
impl std::fmt::Display for XCoordinate {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}.{}.{}", self.chapter, self.section, self.scroll);
  }
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
#[derive(Default, Debug, PartialEq, Copy, Clone)]
#[derive(impl_new::New)]
pub struct Coordinate {
  pub z: ZCoordinate,
  pub y: YCoordinate,
  pub x: XCoordinate,
}
impl std::fmt::Display for Coordinate {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "{}/{}/{}", self.z, self.y, self.x);
  }
}

#[derive(Default, Debug, Clone)]
pub struct PhextParseError;
impl std::error::Error for PhextParseError {}

impl std::fmt::Display for PhextParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "Phext addresses are of the form LB.SF.SR/CN.VM.BK/CH.SN.SC");
  }
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn std::convert::TryFrom
/// ----------------------------------------------------------------------------------------------------------
impl std::convert::TryFrom<&str> for Coordinate {
  type Error = PhextParseError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
      let parts: Vec<&str> = value.split('/').collect();
      let error: PhextParseError = Default::default();
      if parts.len() != 3 {        
        return Err(error);
      }
      let z: Vec<&str> = parts[0].split('.').collect();
      let y: Vec<&str> = parts[1].split('.').collect();
      let x: Vec<&str> = parts[2].split('.').collect();
      if z.len() != 3 || y.len() != 3 || x.len() != 3 {
        return Err(error);
      }
      let mut result: Coordinate = Default::default();
      result.z.library = z[0].parse::<usize>().expect("Library missing");
      result.z.shelf = z[1].parse::<usize>().expect("Shelf missing");
      result.z.series = z[2].parse::<usize>().expect("Series missing");
      result.y.collection = y[0].parse::<usize>().expect("Collection missing");
      result.y.volume = y[1].parse::<usize>().expect("Volume missing");
      result.y.book = y[2].parse::<usize>().expect("Book missing");
      result.x.chapter = x[0].parse::<usize>().expect("Chapter missing");
      result.x.section = x[1].parse::<usize>().expect("Section missing");
      result.x.scroll = x[2].parse::<usize>().expect("Scroll missing");
      return Ok(result);
    }
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn get_subspace_coordinates
///
/// finds the start and end offsets for the given coordinate
/// ----------------------------------------------------------------------------------------------------------
pub fn get_subspace_coordinates(subspace: &[u8], target: Coordinate) -> (usize, usize, Coordinate) {
  let mut walker: Coordinate = default_coordinate();
  let mut best: Coordinate = default_coordinate();
  let mut subspace_index: usize = 0;
  let mut stage: u8 = 0;
  let mut start: usize = 0;
  let mut end: usize = 0;
  let mut found = false;

  let mut nearest: Coordinate = null_coordinate();

  for ptr in subspace {
    let next: char = *ptr as char;

    let mut dimension_break = false;

    if next == SCROLL_BREAK {
      walker.scroll_break();
      if stage == 1 { stage = 2; end = subspace_index; }
      subspace_index += 1;
      dimension_break = true;
    }

    if next == SECTION_BREAK {
      walker.section_break();
      if stage == 1 { stage = 2; end = subspace_index; }
      subspace_index += 1;
      dimension_break = true;
    }

    if next == CHAPTER_BREAK {
      walker.chapter_break();
      if stage == 1 { stage = 2; end = subspace_index; }
      subspace_index += 1;
      dimension_break = true;
    }

    if next == BOOK_BREAK {
      walker.book_break();
      if stage == 1 { stage = 2; end = subspace_index; }
      subspace_index += 1;
      dimension_break = true;
    }

    if next == VOLUME_BREAK {
      walker.volume_break();
      if stage == 1 { stage = 2; end = subspace_index; }
      subspace_index += 1;
      dimension_break = true;
    }

    if next == COLLECTION_BREAK {
      walker.collection_break();
      if stage == 1 { stage = 2; end = subspace_index; }
      subspace_index += 1;
      dimension_break = true;
    }

    if next == SERIES_BREAK {
      walker.series_break();
      if stage == 1 { stage = 2; end = subspace_index; }
      subspace_index += 1;
      dimension_break = true;
    }

    if next == SHELF_BREAK {
      walker.shelf_break();
      if stage == 1 { stage = 2; end = subspace_index; }
      subspace_index += 1;
      dimension_break = true;
    }

    if next == LIBRARY_BREAK {
      walker.library_break();
      if stage == 1 { stage = 2; end = subspace_index; }
      subspace_index += 1;
      dimension_break = true;
    }

    if walker.z.library <= target.z.library {
      nearest.z.library = subspace_index;
      if walker.z.library == target.z.library {
        best = walker;
      }

      if walker.z.shelf <= target.z.shelf {
        nearest.z.shelf = subspace_index;
        if walker.z.library == target.z.library &&
           walker.z.shelf == target.z.shelf {
          best = walker;
        }

        if walker.z.series <= target.z.series {
          nearest.z.series = subspace_index;
          if walker.z.library == target.z.library &&
             walker.z.shelf == target.z.shelf &&
             walker.z.series == target.z.series {
            best = walker;
          }

          if walker.y.collection <= target.y.collection {
            nearest.y.collection = subspace_index;
            if walker.z.library == target.z.library &&
               walker.z.shelf == target.z.shelf &&
               walker.z.series == target.z.series &&
               walker.y.collection == target.y.collection {
              best = walker;
            }

            if walker.y.volume <= target.y.volume {
              nearest.y.volume = subspace_index;
              if walker.z.library == target.z.library &&
                 walker.z.shelf == target.z.shelf &&
                 walker.z.series == target.z.series &&
                 walker.y.collection == target.y.collection &&
                 walker.y.volume == target.y.volume {
                best = walker;
              }

              if walker.y.book <= target.y.book {
                nearest.y.book = subspace_index;
                if walker.z.library == target.z.library &&
                   walker.z.shelf == target.z.shelf &&
                   walker.z.series == target.z.series &&
                   walker.y.collection == target.y.collection &&
                   walker.y.volume == target.y.volume &&
                   walker.y.book == target.y.book {
                  best = walker;
                }

                if walker.x.chapter <= target.x.chapter {
                  nearest.x.chapter = subspace_index;
                  if walker.z.library == target.z.library &&
                     walker.z.shelf == target.z.shelf &&
                     walker.z.series == target.z.series &&
                     walker.y.collection == target.y.collection &&
                     walker.y.volume == target.y.volume &&
                     walker.y.book == target.y.book &&
                     walker.x.chapter == target.x.chapter {
                    best = walker;
                  }

                  if walker.x.section <= target.x.section {
                    nearest.x.section = subspace_index;
                    if walker.z.library == target.z.library &&
                       walker.z.shelf == target.z.shelf &&
                       walker.z.series == target.z.series &&
                       walker.y.collection == target.y.collection &&
                       walker.y.volume == target.y.volume &&
                       walker.y.book == target.y.book &&
                       walker.x.chapter == target.x.chapter &&
                       walker.x.section == target.x.section {
                      best = walker;
                    }

                    if walker.x.scroll <= target.x.scroll {
                      nearest.x.scroll = subspace_index;
                      if walker.z.library == target.z.library &&
                         walker.z.shelf == target.z.shelf &&
                         walker.z.series == target.z.series &&
                         walker.y.collection == target.y.collection &&
                         walker.y.volume == target.y.volume &&
                         walker.y.book == target.y.book &&
                         walker.x.chapter == target.x.chapter &&
                         walker.x.section == target.x.section &&
                         walker.x.scroll == target.x.scroll {
                        best = walker;
                        found = true;
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    if dimension_break {
      continue;
    }
    
    if stage == 0 && target == walker {
      start = subspace_index;
      stage = 1;
    }

    subspace_index += 1;

  }

  if stage == 0 {
    println!("Applying partial match...");
    if nearest.x.scroll > 0 {
      start = nearest.x.scroll;
      println!("Picked scroll = {}", start);
    } else if nearest.x.section > 0 {
      start = nearest.x.section;
      println!("Picked section = {}", start);
    } else if nearest.x.chapter > 0 {
      start = nearest.x.chapter;
      println!("Picked chapter = {}", start);
    } else if nearest.y.book > 0 {
      start = nearest.y.book;
      println!("Picked book = {}", start);
    } else if nearest.y.volume > 0 {
      start = nearest.y.volume;
      println!("Picked volume = {}", start);
    } else if nearest.y.collection > 0 {
      start = nearest.y.collection;
      println!("Picked collection = {}", start);
    } else if nearest.z.series > 0 {
      start = nearest.z.series;
      println!("Picked series = {}", start);
    } else if nearest.z.shelf > 0 {
      start = nearest.z.shelf;
      println!("Picked shelf = {}", start);
    } else if nearest.z.library > 0 {
      start = nearest.z.library;
      println!("Picked library = {}", start);
    }

    if target.z.library >= walker.z.library &&
       target.z.shelf >= walker.z.shelf &&
       target.z.series >= walker.z.series &&
       target.y.collection >= walker.y.collection &&
       target.y.volume >= walker.y.volume &&
       target.y.book >= walker.y.book &&
       target.x.chapter >= walker.x.chapter &&
       target.x.section >= walker.x.section &&
       target.x.scroll >= walker.x.scroll {
      best = walker;      
    }

    println!("Selected index={}, target={}, walker={}, best={}", start, target.to_string(), walker.to_string(), best.to_string());

    end = start;
  }

  if end == 0 && start > 0
  {
    end = subspace.len() as usize;
  }

  return (start, end, best);
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn insert
///
/// inserts the content specified in `scroll` at the coordinate within `phext` specified by `location`
/// ----------------------------------------------------------------------------------------------------------
pub fn insert(phext: &str, location: Coordinate, scroll: &str) -> String {
  let bytes = phext.as_bytes();
  let parts = get_subspace_coordinates(bytes, location);
  let mut end: usize = parts.1 + 1;
  let mut fixup: Vec<u8> = vec![];
  let mut subspace_coordinate: Coordinate = parts.2;

  println!("Inserting at {}.{}.{}/{}.{}.{}/{}.{}.{}", subspace_coordinate.z.library, subspace_coordinate.z.shelf, subspace_coordinate.z.series,
  subspace_coordinate.y.collection, subspace_coordinate.y.volume, subspace_coordinate.y.book,
  subspace_coordinate.x.chapter, subspace_coordinate.x.section, subspace_coordinate.x.scroll);
  println!("Navigating to {}.{}.{}/{}.{}.{}/{}.{}.{}", location.z.library, location.z.shelf, location.z.series,
location.y.collection, location.y.volume, location.y.book, location.x.chapter, location.x.section, location.x.scroll);

  while subspace_coordinate.z.library < location.z.library {
    fixup.push(LIBRARY_BREAK as u8);
    subspace_coordinate.library_break();
  }
  while subspace_coordinate.z.shelf < location.z.shelf {
    fixup.push(SHELF_BREAK as u8);
    subspace_coordinate.shelf_break();
  }
  while subspace_coordinate.z.series < location.z.series {
    fixup.push(SERIES_BREAK as u8);
    subspace_coordinate.series_break();
  }
  while subspace_coordinate.y.collection < location.y.collection {
    fixup.push(COLLECTION_BREAK as u8);
    subspace_coordinate.collection_break();
  }
  while subspace_coordinate.y.volume < location.y.volume {
    fixup.push(VOLUME_BREAK as u8);
    subspace_coordinate.volume_break();
  }
  while subspace_coordinate.y.book < location.y.book {
    fixup.push(BOOK_BREAK as u8);
    subspace_coordinate.book_break();
  }
  while subspace_coordinate.x.chapter < location.x.chapter {
    fixup.push(CHAPTER_BREAK as u8);
    subspace_coordinate.chapter_break();
  }
  while subspace_coordinate.x.section < location.x.section {
    fixup.push(SECTION_BREAK as u8);
    subspace_coordinate.section_break();
  }
  while subspace_coordinate.x.scroll < location.x.scroll {
    fixup.push(SCROLL_BREAK as u8);
    subspace_coordinate.scroll_break();
  }
  let text: std::slice::Iter<u8> = scroll.as_bytes().iter();
  let max = bytes.len();
  if end > max { end = max; }
  let left = &bytes[..end];
  let right = &bytes[end..];
  let temp:Vec<u8> = left.iter().chain(fixup.iter()).chain(text).chain(right.iter()).cloned().collect();
  let result: String = String::from_utf8(temp).expect("invalid utf8");
  return result;
}

/// ----------------------------------------------------------------------------------------------------------
/// @fn fetch
///
/// retrieves the plain text string located at the given coordinates.
/// important: this can be optimized with hash tables and memo-ized parsing - for now let's keep it simple
/// see my C# implementation in https://github.com/wbic16/terse-editor if you want to do that
///
/// @param phext  the raw phext buffer to search
/// @param coord  coordinate to select the scroll from
/// ----------------------------------------------------------------------------------------------------------
pub fn fetch(phext: &str, target: Coordinate) -> String {
  let bytes = phext.as_bytes();
  let parts = get_subspace_coordinates(bytes, target);

  let start = parts.0 as usize;
  let end = parts.1 as usize;

  if end > start
  {
    let glyphs: usize = end - start;
    let temp: Vec<u8> = bytes.iter().skip(start).take(glyphs).cloned().collect();
    let result: String = String::from_utf8(temp).expect("invalid utf8");
    return result;
  }

  return "".to_owned();
}

/// ----------------------------------------------------------------------------------------------------------
pub fn locate(phext: &str, target: &str) -> String {  
  return fetch(phext, to_coordinate(target));
}

/// ----------------------------------------------------------------------------------------------------------
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
fn null_coordinate() -> Coordinate {
  let coord = Coordinate {
    z: ZCoordinate {
      library: 0,
      shelf: 0,
      series: 0
    },
    y: YCoordinate {
      collection: 0,
      volume: 0,
      book: 0
    },
    x: XCoordinate {
      chapter: 0,
      section: 0,
      scroll: 0
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

    if byte == ADDRESS_MICRO_BREAK || byte == ADDRESS_MACRO_BREAK || byte == ADDRESS_MACRO_ALT {            
      match index {
        1 => { result.z.library = value as usize; index += 1; },
        2 => { result.z.shelf = value as usize; index += 1; },
        3 => { result.z.series = value as usize; index += 1; },
        4 => { result.y.collection = value as usize; index += 1; },
        5 => { result.y.volume = value as usize; index += 1; },
        6 => { result.y.book = value as usize; index += 1; },
        7 => { result.x.chapter = value as usize; index += 1; },
        8 => { result.x.section = value as usize; index += 1; },
        _ => {}
      }
      value = 0;
    }

    if byte >= 0x30 && byte <= 0x39
    {
      value = exp * value + ((byte - 0x30) as u32);
      if index == 0 { index = 1; }
    }
  }

  result.x.scroll = value as usize;

  return result;
}

fn validate_dimension_index(index: usize) -> bool {
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
  fn advance_coordinate(index: usize) -> usize {
    let next: usize = index + 1;
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
    self.y.collection = 1;
    self.y.volume = 1;
    self.y.book = 1;
    self.x.chapter = 1;
    self.x.section = 1;
    self.x.scroll = 1;
  }
}

use rocket::request::FromParam;

use crate::default;

impl<'r> FromParam<'r> for Coordinate {
  type Error = &'r str;

  fn from_param(param: &'r str) -> Result<Self, Self::Error> {
      if !param.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(&param);
      }

      let test: Coordinate = to_coordinate(param);
      if !test.validate_coordinate() {
        return Err(&param);
      }

      return Ok(test);
  }
}