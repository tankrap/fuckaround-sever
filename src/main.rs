#![deny(clippy::implicit_return)]
#![feature(macro_metavar_expr_concat)]
#![feature(try_trait_v2)]
#![feature(associated_type_defaults)]

mod app;
mod config;
mod db;
mod emailer;
mod errors;
mod models;

fn main()
{
    println!("Hello, world!");
}
