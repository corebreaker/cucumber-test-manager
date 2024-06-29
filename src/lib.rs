#[cfg(feature = "macro")]
extern crate proc_macro;

mod definition;
mod result;
mod trellis;

#[cfg(feature = "macro")]
mod r#macro;

pub mod spawners;

pub use self::{definition::CucumberTest, trellis::CucumberTrellis};

#[cfg(feature = "macro")]
pub use r#macro::cucumber_test;
