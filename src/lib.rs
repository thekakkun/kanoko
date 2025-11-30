//! A Rust library for generative art (initially) based off of
//! [a traditional Japanese tie-dye pattern](https://en.wikipedia.org/wiki/Shibori#Kanoko_shibori).
//!
//! # Description
//!
//! Define a [`Canvas`] with a [`PointSet`](point_set). Then add [`Shapes`](shape) to be drawn on
//! each point in the `PointSet`.
//!
//! Each point in the `PointSet` has an associated [`Index`](point_set::PointSet::Index). This gets
//! passed to the `Shape` so that you can control if and how the shape gets rendered depending on
//! its `Index`.
//!
//! # Example
//!
//! ```rust,no_run
#![doc = include_str!("../examples/kanoko.rs")]
//! ```

mod canvas;
mod color;
pub mod geometry;
pub mod point_set;
pub mod shape;

pub use canvas::Canvas;
pub use color::Color;
