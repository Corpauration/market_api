pub mod dev;

pub trait Provider<Value> {
    fn provide(self) -> Value;
}

// Note: Some kind fo support for frunk's HList, or a wrapper around it, to be provider of its elements was intended. But as of Rust 1.98.1, there is no stable way to implement that in any useful way.
