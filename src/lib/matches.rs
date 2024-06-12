pub trait Matches<T> {
    fn matches(&self, other: T) -> bool;
}