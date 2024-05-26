#[macro_export]
macro_rules! printnnl {
    ($($arg:tt)*) => {
        color_print::cprint!($($arg)*);
        io::stdout().flush().unwrap();
    }
}

#[macro_export]
macro_rules! builtins_map {
    () => (std::collection::HashMap::new());
    ($($key:expr => $value:expr), + $(,)?) => ({
        let mut map: std::collections::HashMap<&str, fn(crate::types::Args)> = std::collections::HashMap::new();
        $(map.insert($key, $value);)+
        map
    })
}
