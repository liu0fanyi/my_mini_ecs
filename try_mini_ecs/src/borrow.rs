use std::sync::atomic::AtomicUsize;

/// An atomic integer used to dynamicaly enforce borrowing rules
///
/// The most significant bit is used to track mutable borrow, and the rest is a
/// counter for immutable borrows.
///
/// It has four possible states:
///  - `0b00000000...` the counter isn't mut borrowed, and ready for borrowing
///  - `0b0_______...` the counter isn't mut borrowed, and currently borrowed
///  - `0b10000000...` the counter is mut borrowed
///  - `0b1_______...` the counter is mut borrowed, and some other thread is trying to borrow
pub struct AtomicBorrow(AtomicUsize);

