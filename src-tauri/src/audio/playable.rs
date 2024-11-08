pub trait Playable<T> {
    fn next_sample(&mut self) -> Option<(T, T)>;
}
