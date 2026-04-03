//! # Plane

use crate::HitPolicy;
use crate::errors::*;
use crate::point::Point;
use crate::rect::Rect;
use dsntk_common::Result;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

/// Cell of the plane.
#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
  /// Rectangular region with unique number, bounded by coordinates defined in **Rect**
  /// and containing the text taken from original decision table source file.
  Region(usize, Rect, String),
  /// Vertical double line between the input clauses and output clauses, continuing
  /// between the input entries and the output entries (DMN 1.2 clause 8.2.1)
  /// in the horizontally oriented decision table (rules as rows).
  VerticalOutputDoubleLine,
  /// Vertical double line between the output clauses and the annotation clauses, continuing
  /// between the output entries and the annotation entries (DMN 1.2 clause 8.2.1)
  /// in the horizontally oriented decision table (rules as rows).
  VerticalAnnotationDoubleLine,
  /// Horizontal double line between the input clauses and output clauses, continuing
  /// between the input entries and the output entries (DMN 1.2 clause 8.2.1)
  /// in the vertically oriented decision table (rules as columns).
  HorizontalOutputDoubleLine,
  /// Horizontal double line between the output clauses and the annotation clauses, continuing
  /// between the output entries and the annotation entries (DMN 1.2 clause 8.2.1)
  /// in the vertically oriented decision table (rules as columns).
  HorizontalAnnotationsDoubleLine,
  /// Crossing between the vertical output double line or horizontal output double line
  /// and the double line separating input clauses and input entries, continuing between
  /// the output clauses and output entries, and continuing between the annotation clauses
  /// and the annotation entries (DMN 1.2 clause 8.2.1). Each decision table must have this crossing.
  MainDoubleCrossing,
  /// Crossing between the vertical annotation double line and the double line separating
  /// input clauses and input entries, continuing between the output clauses and output entries,
  /// and continuing between the annotation clauses and the annotation entries (DMN 1.2 clause 8.2.1).
  /// Each horizontally oriented decision table must have this crossing when optional annotations are present.
  HorizontalDoubleCrossing,
  /// Crossing between the horizontal annotation double line and the double line separating
  /// input clauses and input entries, continuing between the output clauses and output entries,
  /// and continuing between the annotation clauses and the annotation entries (DMN 1.2 clause 8.2.1).
  /// Each vertically oriented decision table must have this crossing when optional annotations are present.
  VerticalDoubleCrossing,
}

impl Cell {
  /// Checks whether this cell is a main double-crossing.
  pub fn is_main_double_crossing(&self) -> bool {
    matches!(self, Cell::MainDoubleCrossing)
  }

  /// Checks whether this cell is a horizontal double-crossing.
  pub fn is_horz_double_crossing(&self) -> bool {
    matches!(self, Cell::HorizontalDoubleCrossing)
  }

  /// Checks whether this cell is a vertical double-crossing.
  pub fn is_vert_double_crossing(&self) -> bool {
    matches!(self, Cell::VerticalDoubleCrossing)
  }
}

/// Placement of the hit policy marker in plane.
#[derive(Debug, PartialEq, Eq)]
pub enum HitPolicyPlacement {
  /// Hit policy marker is placed in the top-left corner of the plane.
  /// This placement is valid for horizontal decision tables (rules as rows).
  TopLeft(HitPolicy),
  /// Hit policy marker is placed in the bottom-left corner of the plane.
  /// This placement is valid for vertical decision tables (rules as columns).
  BottomLeft(HitPolicy),
  /// Hit policy marker was not found, neither in the top-left nor in the bottom-left corner.
  /// This is valid for decision tables with rules as crosstab.
  NotPresent,
}

impl HitPolicyPlacement {
  /// Checks whether this policy placement is the [HitPolicyPlacement::TopLeft] variant.
  pub fn is_top_left(&self) -> bool {
    matches!(self, HitPolicyPlacement::TopLeft(_))
  }

  /// Checks whether this policy placement is the [HitPolicyPlacement::BottomLeft] variant.
  pub fn is_bottom_left(&self) -> bool {
    matches!(self, HitPolicyPlacement::BottomLeft(_))
  }

  /// Returns hit policy associated with this placement.
  pub fn hit_policy(&self) -> HitPolicy {
    match self {
      HitPolicyPlacement::TopLeft(hit_policy) => *hit_policy,
      HitPolicyPlacement::BottomLeft(hit_policy) => *hit_policy,
      HitPolicyPlacement::NotPresent => HitPolicy::Unique,
    }
  }
}

/// Placement of the rules in plane.
#[derive(Debug, PartialEq, Eq)]
pub enum RuleNumbersPlacement {
  /// In horizontal decision tables (rules as rows), rule numbers are placed
  /// in the first column of the plane, on the left edge, below horizontal double
  /// line separating input clauses and input entries.
  LeftBelow(usize),
  /// In vertical decision tables (rules as columns), rule numbers are places
  /// in the last row of the plane, on the right side, after vertical double line
  /// separating input clauses and input entries.
  RightAfter(usize),
  /// In decision tables with rules as crosstab, there are no rule numbers.
  NotPresent,
}

impl RuleNumbersPlacement {
  /// Checks whether this rule numbers placement is the **LeftBelow** variant.
  pub fn is_left_below(&self) -> bool {
    matches!(self, RuleNumbersPlacement::LeftBelow(_))
  }

  /// Checks whether this rule numbers placement is the **RightAfter** variant.
  pub fn is_right_after(&self) -> bool {
    matches!(self, RuleNumbersPlacement::RightAfter(_))
  }

  /// Checks whether this rule numbers placement is the **NotPresent** variant.
  pub fn is_not_present(&self) -> bool {
    matches!(self, RuleNumbersPlacement::NotPresent)
  }

  /// Returns the amount of recognized rules numbers.
  pub fn rule_count(&self) -> usize {
    match self {
      RuleNumbersPlacement::LeftBelow(count) => *count,
      RuleNumbersPlacement::RightAfter(count) => *count,
      RuleNumbersPlacement::NotPresent => 0,
    }
  }
}

/// Plane.
#[derive(Debug, Default)]
pub struct Plane {
  /// Matrix of cells.
  content: Vec<Vec<Cell>>,
}

impl Plane {
  /// Adds a new `cell` at the end of the specified `row`.
  pub fn add_cell(&mut self, row: usize, cell: Cell) {
    if row == self.content.len() {
      self.content.push(vec![]);
    }
    self.content[row].push(cell)
  }

  /// Returns a `cell` placed in specified `row` and `col`.
  pub fn cell(&self, row: usize, col: usize) -> Result<&Cell> {
    if self.content.is_empty() {
      return Err(err_plane_is_empty());
    }
    if row >= self.content.len() {
      return Err(err_plane_row_is_out_of_range());
    }
    if col >= self.content[row].len() {
      return Err(err_plane_column_is_out_of_range());
    }
    Ok(&self.content[row][col])
  }

  /// Returns the text contained in a region pointed by `row` and `col` coordinates.
  pub fn region_text(&self, row: usize, col: usize) -> Result<String> {
    let cell = self.cell(row, col)?;
    if let Cell::Region(_, _, text) = cell {
      Ok(text.trim().to_string())
    } else {
      Err(err_plane_cell_is_not_region(&format!("row={row} col={col} cell={cell:?}")))
    }
  }

  /// Returns the region number of a region pointed by `row` and `col`.
  pub fn region_number(&self, row: usize, col: usize) -> Result<usize> {
    let cell = self.cell(row, col)?;
    if let Cell::Region(number, _, _) = cell {
      Ok(*number)
    } else {
      Err(err_plane_cell_is_not_region(&format!("row={row} col={col} cell={cell:?}")))
    }
  }

  /// Returns the number of cells in specified `row`.
  pub fn row_len(&self, row: usize) -> usize {
    self.content[row].len()
  }

  /// Removes the first column from the plane.
  pub fn remove_first_column(&mut self) {
    for row in &mut self.content {
      if !row.is_empty() {
        row.remove(0);
      }
    }
  }

  /// Removes the last row from the plane.
  pub fn remove_last_row(&mut self) {
    if !self.content.is_empty() {
      self.content.remove(self.content.len() - 1);
    }
  }

  /// Verifies if all cells in the specified rectangle are pointing to the same regions.
  /// The same (or equal) regions have the same region number.
  pub fn equal_regions(&self, r: &Rect) -> Result<bool> {
    let number = self.region_number(r.top, r.left)?;
    for row in r.top..r.bottom {
      for col in r.left..r.right {
        let n = self.region_number(row, col)?;
        if n != number {
          return Ok(false);
        }
      }
    }
    Ok(true)
  }

  /// Verifies if columns in a rectangle have equal regions.
  /// Different columns may have different regions.
  pub fn equal_regions_in_columns(&self, rect: &Rect) -> Result<bool> {
    for x in rect.left..rect.right {
      let r = Rect::new(x, rect.top, x + 1, rect.bottom);
      if !self.equal_regions(&r)? {
        return Ok(false);
      }
    }
    Ok(true)
  }

  /// Verifies if all cells in the specified rectangle point to unique regions.
  /// Region uniqueness is checked by region number.
  pub fn unique_regions(&self, r: &Rect) -> Result<bool> {
    let mut numbers = HashSet::new();
    for row in r.top..r.bottom {
      for col in r.left..r.right {
        let n = self.region_number(row, col)?;
        if numbers.contains(&n) {
          return Ok(false);
        } else {
          numbers.insert(n);
        }
      }
    }
    Ok(true)
  }

  /// Verifies if columns in a rectangle point to unique regions.
  /// Different columns may have equal regions.
  pub fn unique_regions_in_columns(&self, r: &Rect) -> Result<bool> {
    for x in r.left..r.right {
      let r = Rect::new(x, r.top, x + 1, r.bottom);
      if !self.unique_regions(&r)? {
        return Ok(false);
      }
    }
    Ok(true)
  }

  /// Returns the number of columns in plane (the width of the plane).
  pub fn width(&self) -> usize {
    if self.content.is_empty() { 0 } else { self.content[0].len() }
  }

  /// Returns the number of rows in plane (the height of the plane).
  pub fn height(&self) -> usize {
    self.content.len()
  }

  /// Pivots the plane.
  pub fn pivot(&mut self) {
    let mut pivot_content: Vec<Vec<Cell>> = vec![];
    while !self.content[0].is_empty() {
      pivot_content.push(vec![]);
      for row in 0..self.content.len() {
        let cell = self.content[row].remove(0);
        let new_cell = match cell {
          Cell::Region(n, r, t) => Cell::Region(n, r, t),
          Cell::HorizontalOutputDoubleLine => Cell::VerticalOutputDoubleLine,
          Cell::VerticalOutputDoubleLine => Cell::HorizontalOutputDoubleLine,
          Cell::HorizontalAnnotationsDoubleLine => Cell::VerticalAnnotationDoubleLine,
          Cell::VerticalAnnotationDoubleLine => Cell::HorizontalAnnotationsDoubleLine,
          Cell::MainDoubleCrossing => Cell::MainDoubleCrossing,
          Cell::HorizontalDoubleCrossing => Cell::VerticalDoubleCrossing,
          Cell::VerticalDoubleCrossing => Cell::HorizontalDoubleCrossing,
        };
        pivot_content.last_mut().unwrap().push(new_cell);
      }
    }
    self.content = pivot_content;
  }

  /// Returns rectangle containing input clauses in horizontal table.
  pub fn horz_input_clause_rect(&self) -> Result<Rect> {
    let p = self.main_double_crossing()?;
    Ok(Rect::new(0, 0, p.x, p.y))
  }

  /// Returns a rectangle containing input entries in horizontal table.
  pub fn horz_input_entries_rect(&self) -> Result<Rect> {
    let p = self.main_double_crossing()?;
    Ok(Rect::new(0, p.y + 1, p.x, self.height()))
  }

  /// Returns a rectangle containing output clauses in horizontal table.
  pub fn horz_output_clause_rect(&self) -> Result<Rect> {
    let p = self.main_double_crossing()?;
    if let Some(q) = self.horizontal_double_crossing() {
      Ok(Rect::new(p.x + 1, 0, q.x, p.y))
    } else {
      Ok(Rect::new(p.x + 1, 0, self.width(), p.y))
    }
  }

  /// Returns a rectangle containing output entries in horizontal table.
  pub fn horz_output_entries_rect(&self) -> Result<Rect> {
    let p = self.main_double_crossing()?;
    if let Some(q) = self.horizontal_double_crossing() {
      Ok(Rect::new(p.x + 1, p.y + 1, q.x, self.height()))
    } else {
      Ok(Rect::new(p.x + 1, p.y + 1, self.width(), self.height()))
    }
  }

  /// Returns a rectangle containing annotation clauses in horizontal table.
  pub fn horz_annotation_clauses_rect(&self) -> Result<Rect> {
    if let Some(p) = self.horizontal_double_crossing() {
      Ok(Rect::new(p.x + 1, 0, self.width(), p.y))
    } else {
      Ok(Rect::default())
    }
  }

  /// Returns a rectangle containing output entries in horizontal table.
  pub fn horz_annotation_entries_rect(&self) -> Result<Rect> {
    if let Some(p) = self.horizontal_double_crossing() {
      Ok(Rect::new(p.x + 1, p.y + 1, self.width(), self.height()))
    } else {
      Ok(Rect::default())
    }
  }

  /// Checks if the plane contains main double-crossing.
  /// If the main double-crossing was found on this plane, its position is returned.
  pub fn main_double_crossing(&self) -> Result<Point> {
    for (y, row) in self.content.iter().enumerate() {
      for (x, cell) in row.iter().enumerate() {
        if cell.is_main_double_crossing() {
          return Ok(Point::new(x, y));
        }
      }
    }
    Err(err_plane_no_main_double_crossing())
  }

  /// Checks if the plane contains horizontal double-crossing.
  /// If the horizontal double-crossing was found on this plane, its position is returned.
  pub fn horizontal_double_crossing(&self) -> Option<Point> {
    for (y, row) in self.content.iter().enumerate() {
      for (x, cell) in row.iter().enumerate() {
        if cell.is_horz_double_crossing() {
          return Some(Point::new(x, y));
        }
      }
    }
    None
  }

  /// Checks if the plane contains vertical double-crossing.
  /// If the vertical double-crossing was found on this plane, its position is returned.
  pub fn vertical_double_crossing(&self) -> Option<Point> {
    for (y, row) in self.content.iter().enumerate() {
      for (x, cell) in row.iter().enumerate() {
        if cell.is_vert_double_crossing() {
          return Some(Point::new(x, y));
        }
      }
    }
    None
  }

  /// Recognizes the hit policy placement. In properly defined decision table
  /// and thus in the properly defined plane, the hit policy is placed
  /// either in the top-left or in the bottom-left corner.
  /// This rule applies to horizontal and vertical decision tables respectively.
  /// Decision tables with crosstab rules have no hit policy.
  pub fn recognize_hit_policy_placement(&self) -> Result<HitPolicyPlacement> {
    if self.content.is_empty() {
      return Err(err_plane_is_empty());
    }
    // check if the hit policy is placed in the top-left corner of the plane
    if let Cell::Region(_, _, text) = &self.content.first().unwrap().first().unwrap()
      && let Ok(hit_policy) = HitPolicy::try_from(text.as_str())
    {
      // hit policy is placed in top-left corner
      return Ok(HitPolicyPlacement::TopLeft(hit_policy));
    }
    // check if the hit policy is placed in the bottom-left corner of the plane
    if let Cell::Region(_, _, text) = &self.content.last().unwrap().first().unwrap()
      && let Ok(hit_policy) = HitPolicy::try_from(text.as_str())
    {
      // hit policy is placed in bottom-left corner
      return Ok(HitPolicyPlacement::BottomLeft(hit_policy));
    }
    // hit policy was not found in the top-left or bottom-right corner of the plane
    Ok(HitPolicyPlacement::NotPresent)
  }

  /// Recognizes the placement of the rule numbers in decision table.
  pub fn recognize_rule_numbers_placement(&self) -> Result<RuleNumbersPlacement> {
    match self.recognize_horizontal_rule_numbers() {
      Ok(RuleNumbersPlacement::NotPresent) => self.recognize_vertical_rule_numbers(),
      other => other,
    }
  }

  /// Checks if rule numbers are placed on the left side below horizontal output double line.
  fn recognize_horizontal_rule_numbers(&self) -> Result<RuleNumbersPlacement> {
    let mut row = 0;
    while !self.is_horizontal_output_double_line(row, 0) {
      row += 1;
    }
    row += 1;
    let mut max_rule_number = 0;
    while row < self.content.len() {
      if let Cell::Region(_, _, text) = &self.content[row][0] {
        let text = text.trim();
        if let Ok(rule_number) = usize::from_str(text) {
          if rule_number != max_rule_number + 1 {
            return Err(err_plane_invalid_rule_number(rule_number));
          } else {
            max_rule_number = rule_number;
          }
        } else {
          return Ok(RuleNumbersPlacement::NotPresent);
        }
      } else {
        return Ok(RuleNumbersPlacement::NotPresent);
      }
      row += 1;
    }
    if max_rule_number > 0 {
      Ok(RuleNumbersPlacement::LeftBelow(max_rule_number))
    } else {
      Ok(RuleNumbersPlacement::NotPresent)
    }
  }

  /// Checks if rule numbers are placed on the right side after vertical output double line.
  fn recognize_vertical_rule_numbers(&self) -> Result<RuleNumbersPlacement> {
    let mut col = 0;
    let row = self.content.len() - 1;
    while !self.is_vertical_output_double_line(row, col) {
      col += 1;
    }
    col += 1;
    let mut max_rule_number = 0;
    while col < self.content[row].len() {
      if let Cell::Region(_, _, text) = &self.content[row][col] {
        let text = text.trim();
        if let Ok(rule_number) = usize::from_str(text) {
          if rule_number != max_rule_number + 1 {
            return Err(err_plane_invalid_rule_number(rule_number));
          } else {
            max_rule_number = rule_number;
          }
        } else {
          return Ok(RuleNumbersPlacement::NotPresent);
        }
      } else {
        return Ok(RuleNumbersPlacement::NotPresent);
      }
      col += 1;
    }
    if max_rule_number > 0 {
      Ok(RuleNumbersPlacement::RightAfter(max_rule_number))
    } else {
      Ok(RuleNumbersPlacement::NotPresent)
    }
  }

  /// Checks if the cell pointed by coordinates `row` and `col` is a horizontal output double line.
  fn is_horizontal_output_double_line(&self, row: usize, col: usize) -> bool {
    self.content[row][col] == Cell::HorizontalOutputDoubleLine
  }

  /// Checks if the cell pointed by coordinates `row` and `col` is a vertical output double line.
  fn is_vertical_output_double_line(&self, row: usize, col: usize) -> bool {
    self.content[row][col] == Cell::VerticalOutputDoubleLine
  }
}

impl Display for Plane {
  /// Implements [Display] trait for [Plane].
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut buffer = String::new();
    for row in &self.content {
      for cell in row {
        let text = match cell {
          Cell::Region(n, _, _) => format!(" {number:>0width$x} ", number = n, width = 3),
          Cell::HorizontalOutputDoubleLine => "═════".to_string(),
          Cell::HorizontalAnnotationsDoubleLine => "─────".to_string(),
          Cell::VerticalOutputDoubleLine => "║".to_string(),
          Cell::VerticalAnnotationDoubleLine => "│".to_string(),
          Cell::MainDoubleCrossing => "╬".to_string(),
          Cell::HorizontalDoubleCrossing => "╪".to_string(),
          Cell::VerticalDoubleCrossing => "╫".to_string(),
        };
        buffer.push_str(text.as_str());
      }
      buffer.push('\n');
    }
    write!(f, "{buffer}")
  }
}
