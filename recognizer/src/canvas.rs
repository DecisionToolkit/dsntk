//! # Canvas

use crate::errors::*;
use crate::plane::{Cell, Plane};
use crate::point::Point;
use crate::rect::Rect;
use dsntk_common::Result;
use std::cmp::max;

/// Number of layers in canvas.
pub const LAYER_COUNT: usize = 4;

/// Names of layers in canvas.
pub const LAYER_NAMES: [&str; LAYER_COUNT] = ["TEXT", "THIN", "BODY", "GRID"];

/// Layer containing original text of the decision table.
pub const LAYER_TEXT: usize = 0;

/// Layer containing all the regions with single lines.
pub const LAYER_THIN: usize = 1;

/// Layer containing decision table body without information item name.
pub const LAYER_BODY: usize = 2;

/// Layer containing grid of single lines.
pub const LAYER_GRID: usize = 3;

const CHAR_WS: char = ' ';
const CHAR_OUTER: char = '░';

const CORNER_TOP_LEFT: char = '┌';
const CORNER_BOTTOM_RIGHT: char = '┘';

const CORNERS_TOP_LEFT: [char; 4] = ['┌', '├', '┬', '┼'];
const CORNERS_TOP_RIGHT: [char; 4] = ['┐', '┤', '┬', '┼'];
const CORNERS_BOTTOM_RIGHT: [char; 4] = ['┘', '┤', '┴', '┼'];
const CORNERS_BOTTOM_LEFT: [char; 4] = ['└', '├', '┴', '┼'];

type Layer = usize;
type Layers = [char; LAYER_COUNT];

pub struct Canvas {
  /// Content of the canvas build from layers.
  content: Vec<Vec<Layers>>,
  /// Current cursor position in the canvas.
  cursor: Point,
  /// Position of the required double line crossing between inputs and outputs.
  cross: Option<Point>,
  /// Position of the optional double line crossing between outputs and annotations in horizontal decision table.
  cross_horz: Option<Point>,
  /// Position of the optional double line crossing between outputs and annotations in vertical decision table.
  cross_vert: Option<Point>,
  /// Optional information item name.
  pub information_item_name: Option<String>,
  /// Rectangle containing the body of the decision table.
  body_rect: Option<Rect>,
}

impl Canvas {
  /// Prepares a [Canvas] containing the textual definition of decision table.
  /// This function traverses the input text line-by-line and adds non-empty lines to the canvas.
  /// Adding lines to canvas begins with the first line starting with the `┌` character
  /// (U+250C BOX DRAWINGS LIGHT DOWN AND RIGHT), because this is the top-left corner of every decision table.
  /// Adding lines to canvas ends with the line that ends with the `┘` character
  /// (U+2518 BOX DRAWINGS LIGHT UP AND LEFT), because this is the bottom right corner of every decision table.
  /// Shorter lines are filled up with additional characters to form a rectangular area.
  pub fn scan(text: &str) -> Result<Self> {
    #[derive(PartialEq, Eq)]
    enum State {
      Before = 0,
      Appending = 1,
      After = 2,
    }
    let mut state = State::Before;
    let mut max_row_length: usize = 0;
    let mut content = vec![];
    for line in text.lines().map(|line| line.trim()).filter(|line| !line.is_empty()) {
      if state == State::Before && line.starts_with(CORNER_TOP_LEFT) {
        state = State::Appending;
      }
      if state == State::Appending {
        let row = line.chars().map(|ch| [ch, CHAR_WS, CHAR_WS, CHAR_WS]).collect::<Vec<Layers>>();
        max_row_length = max(max_row_length, row.len());
        content.push(row);
      }
      if state == State::Appending && line.ends_with(CORNER_BOTTOM_RIGHT) {
        state = State::After;
      }
    }
    for row in &mut content {
      while row.len() < max_row_length {
        row.push([CHAR_OUTER; LAYER_COUNT]);
      }
    }
    // create the canvas
    let mut canvas = Canvas {
      content,
      cursor: Point::zero(),
      cross: None,
      cross_horz: None,
      cross_vert: None,
      information_item_name: None,
      body_rect: None,
    };
    canvas.recognize_information_item_name()?;
    canvas.recognize_crossings()?;
    canvas.body_rect = Some(canvas.recognize_body_rect()?);
    canvas.prepare_regions(LAYER_TEXT, LAYER_THIN);
    canvas.remove_information_item_region(LAYER_THIN, LAYER_BODY);
    canvas.make_grid(LAYER_BODY, LAYER_GRID);
    Ok(canvas)
  }

  /// Recognizes the information item name.
  fn recognize_information_item_name(&mut self) -> Result<()> {
    // search for information item name in the original text
    let layer = LAYER_TEXT;
    // move cursor to the beginning of the canvas (top-left corner with (0,0) coordinates)
    self.move_to(Point::zero());
    // search for '┌' character representing the top left corner of every the decision table,
    // this character must be present, otherwise the decision table is malformed
    let (_, top_left) = self.search(layer, &['┌'])?;
    // search for the crossing of the top edge with double line (must be present, error otherwise)
    self.search(layer, &['╥']).and_then(|(_, top_edge)| {
      // if the edge is below the corner than the information item region is present
      // so try to recognize the whole information item name
      if top_left.y < top_edge.y {
        // move to the top-left corner of the decision table
        self.move_to(top_left);
        // move right along the top information item name edge until the corner is encountered
        self.search_right(layer, &['┐'], &['─']).and_then(|_| {
          // move down to the nearest crossing
          self.search_down(layer, &['┴', '┤', '┼'], &['│']).and_then(|(_, bottom_right)| {
            // move left until the left body edge
            self.search_left(layer, &['├'], &['─', '┬', '╥']).and_then(|_| {
              // move up until the original top-left corner is reached
              self.search_up(layer, &['┌'], &['│']).and_then(|(_, closing)| {
                // verify if the rectangle may be closed and retrieve the text
                self.close_rectangle(closing, top_left, bottom_right).map(|rect| {
                  let text = self.text_from_rect(layer, &rect);
                  self.information_item_name = Some(text);
                })
              })
            })
          })
        })
      } else {
        // TODO Report an error when the left top corner is below top edge.
        Ok(())
      }
    })
  }

  /// Recognizes crossings.
  fn recognize_crossings(&mut self) -> Result<()> {
    // move cursor to the top-left corner
    self.move_to(Point::zero());
    self.search(LAYER_TEXT, &['╬']).map(|(_, point)| {
      self.cross = Some(point);
      self.move_to(point);
      if let Ok((_, p)) = self.search_right(LAYER_TEXT, &['╬'], &['═', '╪']) {
        self.cross_horz = Some(p)
      }
      self.move_to(point);
      if let Ok((_, p)) = self.search_down(LAYER_TEXT, &['╬'], &['║', '╫']) {
        self.cross_vert = Some(p)
      }
      // TODO Add validation checking whether two crossings exist.
    })
  }

  /// Recognizes a body rectangle.
  fn recognize_body_rect(&mut self) -> Result<Rect> {
    let layer = LAYER_TEXT;
    // move to the top-left corner
    self.move_to(Point::zero());
    // find the first double line crossing
    self.search(layer, &['╬']).and_then(|(_, cross_point)| {
      // move up until the top edge of the body is reached
      self.search_up(layer, &['╥'], &['║', '╫', '╟', '╢']).and_then(|(_, top_point)| {
        // move down until the bottom edge of the body is reached
        self.search_down(layer, &['╨'], &['║', '╫', '╟', '╢', '╬']).and_then(|(_, bottom_point)| {
          // move again to the first crossing
          self.move_to(cross_point);
          // move left until the left edge of the body is reached
          self.search_left(layer, &['╞'], &['═', '╪', '╧', '╤']).and_then(|(_, left_point)| {
            // move right until the right edge of the body is reached
            self
              .search_right(layer, &['╡'], &['═', '╪', '╧', '╤', '╬'])
              .map(|(_, right_point)| Rect::new(left_point.x, top_point.y, right_point.x + 1, bottom_point.y + 1))
          })
        })
      })
    })
  }

  /// Replaces all double lines and double crossings with single lines.
  /// All character other than lines are replaced with whitespace.
  fn prepare_regions(&mut self, src: Layer, dst: Layer) {
    for row in self.content.iter_mut() {
      for col in row.iter_mut() {
        match col[src] {
          '┌' | '┐' | '└' | '┘' | '┬' | '┴' | '─' | '│' | '├' | '┼' | '┤' | ' ' | CHAR_OUTER => col[dst] = col[src],
          '╥' | '╤' => col[dst] = '┬',
          '║' => col[dst] = '│',
          '╨' | '╧' => col[dst] = '┴',
          '═' => col[dst] = '─',
          '╞' | '╟' => col[dst] = '├',
          '╡' | '╢' => col[dst] = '┤',
          '╫' | '╪' | '╬' => col[1] = '┼',
          _ => col[dst] = CHAR_WS,
        }
      }
    }
  }

  /// Removes the information item name region.
  fn remove_information_item_region(&mut self, src: Layer, dst: Layer) {
    if let Some(body_rect) = self.body_rect {
      let (left, top, right, bottom) = body_rect.into();
      if top > 0 {
        for y in 0..top - 1 {
          for x in left..right {
            for layer in dst..LAYER_COUNT {
              self.content[y][x][layer] = CHAR_OUTER;
            }
          }
        }
      }
      for x in left..right {
        match self.content[top][x][src] {
          '├' => self.content[top][x][dst] = '┌',
          '┴' => self.content[top][x][dst] = '─',
          '┤' => self.content[top][x][dst] = '┐',
          '┼' => self.content[top][x][dst] = '┬',
          _ => self.content[top][x][dst] = self.content[top][x][src],
        }
      }
      for y in top + 1..bottom {
        for x in left..right {
          self.content[y][x][dst] = self.content[y][x][src];
        }
      }
    }
  }

  /// Fills the lacking lines to constitute regular grid
  fn make_grid(&mut self, src: Layer, dst: Layer) {
    if let Some(body_rect) = self.body_rect {
      self.copy_layer(src, dst);
      let (left, top, right, bottom) = body_rect.into();
      for y in top..bottom {
        let mut has_horz_line = false;
        for x in left..right {
          if self.content[y][x][dst] == '─' {
            has_horz_line = true;
            break;
          }
        }
        if has_horz_line {
          for x in left..right {
            match self.content[y][x][dst] {
              '│' if x == left => self.content[y][x][dst] = '├',
              '│' if x == right - 1 => self.content[y][x][dst] = '┤',
              '│' => self.content[y][x][dst] = '┼',
              '┤' if x < right - 1 => self.content[y][x][dst] = '┼',
              '├' if x > 0 => self.content[y][x][dst] = '┼',
              CHAR_WS => self.content[y][x][dst] = '─',
              _ => {}
            }
          }
        }
      }
      for x in left..right {
        let mut has_vert_line = false;
        for y in top..bottom {
          if self.content[y][x][dst] == '│' {
            has_vert_line = true;
            break;
          }
        }
        if has_vert_line {
          for y in top..bottom {
            match self.content[y][x][dst] {
              '─' if y == top => self.content[y][x][dst] = '┬',
              '─' if y == bottom - 1 => self.content[y][x][dst] = '┴',
              '─' => self.content[y][x][dst] = '┼',
              '┴' if y < bottom - 1 => self.content[y][x][dst] = '┼',
              '┬' if y > top => self.content[y][x][dst] = '┼',
              CHAR_WS => self.content[y][x][dst] = '│',
              _ => {}
            }
          }
        }
      }
    }
  }

  /// Generates matrix plane describing parts of the analyzed decision table.
  /// The matrix is a rectangular set of cells, where each cell represents
  /// a single graphical element of a decision table like region with text,
  /// double line or double-line crossing.
  pub fn plane(&mut self) -> Result<Plane> {
    let regions = self.recognize_regions()?;
    let mut plane: Plane = Default::default(); // the plane to be generated
    let mut row = 0; // current row index
    let mut col; // current column index
    let mut width = 0; // width of the plane
    let mut cross_col = None; // column index of the main crossing
    let mut cross_horz_col = None; // column index of the annotation crossing in horizontal decision table
    for y in 0..self.content.len() {
      if let Some(cross_point) = self.cross
        && y == cross_point.y
      {
        for i in 0..width {
          if let Some(index) = cross_col
            && index == i
          {
            plane.add_cell(row, Cell::MainDoubleCrossing);
            continue;
          }
          if let Some(index) = cross_horz_col
            && index == i
          {
            plane.add_cell(row, Cell::HorizontalDoubleCrossing);
            continue;
          }
          plane.add_cell(row, Cell::HorizontalOutputDoubleLine);
        }
        row += 1;
      }
      if let Some(cross_vert_point) = self.cross_vert
        && y == cross_vert_point.y
      {
        for i in 0..width {
          if let Some(index) = cross_col
            && index == i
          {
            plane.add_cell(row, Cell::VerticalDoubleCrossing);
            continue;
          }
          plane.add_cell(row, Cell::HorizontalAnnotationsDoubleLine);
        }
        row += 1;
      }
      col = 0;
      let mut matched = false;
      for x in 0..self.content[y].len() {
        if CORNERS_TOP_LEFT.contains(&self.content[y][x][LAYER_GRID]) {
          matched = true;
          if let Some(point) = self.cross
            && x == point.x
          {
            cross_col = Some(col);
            plane.add_cell(row, Cell::VerticalOutputDoubleLine);
            col += 1;
          }
          if let Some(point) = self.cross_horz
            && x == point.x
          {
            cross_horz_col = Some(col);
            plane.add_cell(row, Cell::VerticalAnnotationDoubleLine);
            col += 1;
          }
          let rect = self.recognize_rectangle(LAYER_GRID, Point::new(x, y))?;
          let mut found = false;
          for (i, region) in regions.iter().enumerate() {
            if region.contains(&rect) {
              let text = self.text_from_rect(LAYER_TEXT, region);
              plane.add_cell(row, Cell::Region(i, *region, text));
              found = true;
              break;
            }
          }
          if !found {
            return Err(err_canvas_region_not_found(rect));
          }
          col += 1;
        }
      }
      if matched {
        width = plane.row_len(row);
        row += 1;
      }
    }
    Ok(plane)
  }

  /// Recognizes all rectangular regions in layer.
  fn recognize_regions(&mut self) -> Result<Vec<Rect>> {
    let layer = LAYER_THIN;
    let points = self.find_top_left_corners(layer);
    let mut rectangles = vec![];
    for point in points {
      let rectangle = self.recognize_region(layer, point)?;
      rectangles.push(rectangle);
    }
    Ok(rectangles)
  }

  /// Recognizes single rectangular region in specified **layer**.
  /// Recognition starts from the **top_left** position in layer.
  fn recognize_region(&mut self, layer: Layer, top_left: Point) -> Result<Rect> {
    // start in the top left corner of the rectangle
    self.move_to(top_left);
    // move right to the nearest top-right corner
    self.search_right(layer, &CORNERS_TOP_RIGHT, &['─', '┴']).and_then(|_| {
      // move down to the nearest bottom-right corner
      self.search_down(layer, &CORNERS_BOTTOM_RIGHT, &['│', '├']).and_then(|(_, bottom_right)| {
        // move left to the nearest bottom-left corner
        self.search_left(layer, &CORNERS_BOTTOM_LEFT, &['─', '┬']).and_then(|_| {
          // move up to the nearest top-left corner
          self
            .search_up(layer, &CORNERS_TOP_LEFT, &['│', '┤'])
            .and_then(|(_, closing)| self.close_rectangle(closing, top_left, bottom_right))
        })
      })
    })
  }

  /// Recognizes a rectangle.
  fn recognize_rectangle(&mut self, layer: Layer, top_left: Point) -> Result<Rect> {
    // start in the top left corner of the rectangle
    self.move_to(top_left);
    // move right to the nearest corner, this is the top-right corner of the recognized rectangle
    self.search_right(layer, &['┼', '┬', '┤', '┐'], &['─']).and_then(|_| {
      // move down to the nearest corner, this is the bottom-right corner of the recognized rectangle
      self.search_down(layer, &['┼', '┴', '┤', '┘'], &['│']).and_then(|(_, bottom_right)| {
        // move left to the nearest corner, this is the bottom-left corner of the recognized rectangle
        self.search_left(layer, &['┼', '└', '├', '┴'], &['─']).and_then(|_| {
          // move up to the nearest corner, this is the top-left corner of the recognized rectangle
          self
            .search_up(layer, &['┼', '┬', '├', '┌'], &['│'])
            .and_then(|(_, closing)| self.close_rectangle(closing, top_left, bottom_right))
        })
      })
    })
  }

  /// Verifies id the rectangle is properly closed.
  fn close_rectangle(&self, closing: Point, top_left: Point, bottom_right: Point) -> Result<Rect> {
    if closing == top_left {
      Ok(Rect::new(top_left.x, top_left.y, bottom_right.x + 1, bottom_right.y + 1))
    } else {
      Err(err_canvas_rectangle_not_closed(closing, top_left))
    }
  }

  /// Searches for all characters that constitute a top-left corner of a rectangle.
  /// Position of this character is the starting point for recognizing rectangles.
  /// Returns a vector of all top-left corner points found in specified [Layer].
  fn find_top_left_corners(&self, layer: Layer) -> Vec<Point> {
    let mut points = vec![];
    for y in 0..self.content.len() {
      for x in 0..self.content[y].len() {
        if CORNERS_TOP_LEFT.contains(&self.content[y][x][layer]) {
          points.push(Point::new(x, y));
        }
      }
    }
    points
  }

  /// Moves the cursor to the specified position defined as a [Point].
  fn move_to(&mut self, point: Point) {
    let row_count = self.content.len();
    let y = if point.y < row_count {
      point.y
    } else if row_count > 0 {
      row_count - 1
    } else {
      0
    };
    let x = if let Some(col_count) = self.content.get(y).map(|row| row.len()) {
      if point.x < col_count {
        point.x
      } else if col_count > 0 {
        col_count - 1
      } else {
        0
      }
    } else {
      0
    };
    self.cursor = (x, y).into();
  }

  /// Searches for characters specified in `searched`.
  /// The search starts at the current cursor position and continues from left to right
  /// and from top to bottom inside specified layer of the canvas.
  /// When any of the specified characters is found, the cursor position is updated
  /// and the function returns successfully. Otherwise, this function returns an error.
  fn search(&mut self, layer: Layer, searched: &[char]) -> Result<(char, Point)> {
    // start searching from the current cursor position
    let (col_position, row_position) = self.cursor.into();
    // first, search in the current row starting from current column
    for (row_index, row_data) in self.content.iter().skip(row_position).take(1).enumerate() {
      for (col_index, layers) in row_data.iter().skip(col_position).enumerate() {
        let ch = layers[layer];
        if searched.contains(&ch) {
          self.cursor = (col_index + col_position, row_index + row_position).into();
          return Ok((ch, self.cursor));
        }
      }
    }
    // next, search in the consecutive rows
    for (row_index, row_data) in self.content.iter().skip(row_position + 1).enumerate() {
      for (col_index, layers) in row_data.iter().enumerate() {
        let ch = layers[layer];
        if searched.contains(&ch) {
          self.cursor = (col_index, row_index + row_position + 1).into();
          return Ok((ch, self.cursor));
        }
      }
    }
    // return and error when no character was found
    Err(err_canvas_expected_characters_not_found(searched))
  }

  fn search_up(&mut self, layer: usize, searched: &[char], allowed: &[char]) -> Result<(char, Point)> {
    let (x, mut y) = self.cursor.into();
    while y > 0 {
      y -= 1;
      let ch = self.content[y][x][layer];
      if searched.contains(&ch) {
        self.cursor = Point::new(x, y);
        return Ok((ch, self.cursor));
      }
      if !allowed.contains(&ch) {
        return Err(err_canvas_character_is_not_allowed(ch, allowed));
      }
    }
    Err(err_canvas_expected_characters_not_found(searched))
  }

  fn search_left(&mut self, layer: usize, searched: &[char], allowed: &[char]) -> Result<(char, Point)> {
    let (mut x, y) = self.cursor.into();
    while x > 0 {
      x -= 1;
      let ch = self.content[y][x][layer];
      if searched.contains(&ch) {
        self.cursor = Point::new(x, y);
        return Ok((ch, self.cursor));
      }
      if !allowed.contains(&ch) {
        return Err(err_canvas_character_is_not_allowed(ch, allowed));
      }
    }
    Err(err_canvas_expected_characters_not_found(searched))
  }

  fn search_right(&mut self, layer: usize, searched: &[char], allowed: &[char]) -> Result<(char, Point)> {
    let (mut x, y) = self.cursor.into();
    while x < self.content[y].len() - 1 {
      x += 1;
      let ch = self.content[y][x][layer];
      if searched.contains(&ch) {
        self.cursor = Point::new(x, y);
        return Ok((ch, self.cursor));
      }
      if !allowed.contains(&ch) {
        return Err(err_canvas_character_is_not_allowed(ch, allowed));
      }
    }
    Err(err_canvas_expected_characters_not_found(searched))
  }

  fn search_down(&mut self, layer: usize, searched: &[char], allowed: &[char]) -> Result<(char, Point)> {
    let (x, mut y) = self.cursor.into();
    while y < self.content.len() - 1 {
      y += 1;
      let ch = self.content[y][x][layer];
      if searched.contains(&ch) {
        self.cursor = Point::new(x, y);
        return Ok((ch, self.cursor));
      }
      if !allowed.contains(&ch) {
        return Err(err_canvas_character_is_not_allowed(ch, allowed));
      }
    }
    Err(err_canvas_expected_characters_not_found(searched))
  }

  /// Retrieves the text enclosed inside a rectangle `r` in selected `layer`.
  fn text_from_rect(&self, layer: usize, rect: &Rect) -> String {
    self.content[(rect.top + 1)..(rect.bottom - 1)]
      .iter()
      .map(|row| {
        row[(rect.left + 1)..(rect.right - 1)]
          .iter()
          .map(|layers| layers[layer])
          .collect::<String>()
          .trim()
          .to_string()
      })
      .filter(|s| !s.is_empty())
      .collect::<Vec<String>>()
      .join(" ")
  }

  /// Copies characters from source layer to destination layer.
  fn copy_layer(&mut self, src: Layer, dst: Layer) {
    for y in 0..self.content.len() {
      for x in 0..self.content[y].len() {
        self.content[y][x][dst] = self.content[y][x][src];
      }
    }
  }

  /// Displays the content of the text layer.
  pub fn display_text_layer(&self) {
    self.display_layer(LAYER_TEXT);
  }

  /// Displays the content of the thin lines layer.
  pub fn display_thin_layer(&self) {
    self.display_layer(LAYER_THIN);
  }

  /// Displays the content of the body part of decision table.
  pub fn display_body_layer(&self) {
    self.display_layer(LAYER_BODY);
  }

  /// Displays the content of the grid layer.
  pub fn display_grid_layer(&self) {
    self.display_layer(LAYER_GRID);
  }

  /// Displays the content of the specified layer.
  fn display_layer(&self, l: Layer) {
    if l < LAYER_COUNT {
      println!("{}", LAYER_NAMES[l]);
      for row in &self.content {
        for layers in row {
          print!("{}", layers[l])
        }
        println!()
      }
    }
  }
}
