//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

//! #自分のクレート
//!
//! `my_crate`は、ユーティリティの集まりであり、特定の計算をより便利に行うことができます。

pub mod func {
    /// Adds one to the number given.
    /// 与えられた数値に1を足す。
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(6, rust_tutorial::func::add_one(5));
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
}

mod art;

pub use art::kinds::PrimaryColor;
pub use art::kinds::SecondaryColor;
pub use art::utils;
