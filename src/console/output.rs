#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        let formatted = format!($($arg)*);
        println!("[-] {}", formatted);
    }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let formatted = format!($($arg)*);
        print!("[-] {}", formatted);
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let formatted = format!($($arg)*);
        print!("[{}] {}", $crate::console::color::red("x"), formatted);
    }};
}

#[macro_export]
macro_rules! errorln {
    ($($arg:tt)*) => {{
        let formatted = format!($($arg)*);
        println!("[{}] {}", $crate::console::color::red("x"),formatted);
    }};
}

pub use error;
pub use errorln;
pub use print;
pub use println;
