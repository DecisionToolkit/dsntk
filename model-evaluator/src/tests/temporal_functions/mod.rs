//! Tests for evaluating temporal functions within decision tables.
//!
//! This module contains tests organized by temporal function categories:
//! - date_time_comparisons: Tests for date() and time() functions with comparisons
//! - duration_functions: Tests for duration() function
//! - today_function: Tests for today() function with comparisons
//! - age_calculations: Tests for age calculations using years and months duration

pub mod age_calculations;
pub mod date_time_comparisons;
pub mod duration_functions;
pub mod today_function;
