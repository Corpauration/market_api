pub mod dev;

pub trait Provider<Value> {
    fn provide(self) -> Value;
}
