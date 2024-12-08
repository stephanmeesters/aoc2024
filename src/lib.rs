pub mod template;

// Use this file to add helper functions and additional modules.
//
#[macro_export]
macro_rules! printd {
    ($val:expr) => {
        println!("{:?}", $val);
    };
}

