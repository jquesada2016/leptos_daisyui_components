#![feature(const_mut_refs)]

pub mod button;
pub mod text_input;

pub use button::*;
pub use leptos_daisy_ui_macros::include_component_classes;
pub use text_input::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
  Primary,
  Secondary,
  Accent,
  Info,
  Success,
  Warning,
  Error,
}

impl Color {
  const PRIMARY: &str = "primary";
  const SECONDARY: &str = "secondary";
  const ACCENT: &str = "accent";
  const INFO: &str = "info";
  const SUCCESS: &str = "success";
  const WARNING: &str = "warning";
  const ERROR: &str = "error";
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Size {
  ExtraSmall,
  Small,
  Medium,
  Large,
}

impl Size {
  const LARGE: &str = "lg";
  const MEDIUM: &str = "md";
  const SMALL: &str = "sm";
  const EXTRASMALL: &str = "xs";
}
