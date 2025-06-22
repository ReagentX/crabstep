pub struct Consumed<T> {
    pub value: T,
    pub bytes_consumed: usize,
}

impl<T> Consumed<T> {
    pub fn new(value: T, bytes_consumed: usize) -> Self {
        Consumed {
            value,
            bytes_consumed,
        }
    }

    pub fn map<U, F>(self, f: F) -> Consumed<U>
    where
        F: FnOnce(T) -> U,
    {
        Consumed {
            value: f(self.value),
            bytes_consumed: self.bytes_consumed,
        }
    }
}

impl<T> std::ops::Deref for Consumed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for Consumed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
