#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($( $args:expr ),*) => {};
}
