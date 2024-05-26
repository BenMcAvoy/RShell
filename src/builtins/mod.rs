mod builtins;
mod echo;
mod exit;
mod cd;

pub mod prelude {
    pub use super::builtins::builtins;
    pub use super::echo::echo;
    pub use super::exit::exit;
    pub use super::cd::cd;
}
