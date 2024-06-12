pub trait Matches {
    fn matches(&self, other: T) -> bool;
}