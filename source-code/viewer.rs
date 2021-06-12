pub struct Viewer<T: Clone> {
    data: Vec<T>,
    position: usize,
    mark: usize,
}

impl<T: Clone> Viewer<T> {
    pub fn new(data: Vec<T>) -> Self {
        return Self {
            data: data,
            position: 0,
            mark: 0,
        };
    }

    pub fn next(&mut self) -> Option<T> {
        self.position += 1;

        return match self.data.get(self.position - 1) {
            // NOTE: I clone the data instead of returning a reference to it
            // because it makes easier to use `while let` statements which
            // lock the reference and thus prevent access to other elements
            // of the same object.
            Some(value) => Some((*value).clone()),
            None => None,
        };
    }

    pub fn mark(&mut self) {
        self.mark = self.position.clone();
    }

    pub fn rewind(&mut self) {
        self.position = self.mark.clone();
    }
}
