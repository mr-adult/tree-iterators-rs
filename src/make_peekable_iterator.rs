pub (crate) struct MakePeekableIterator<T: Iterator> {
    iter: T,
    lookahead: Option<T::Item>,
}

impl<T> MakePeekableIterator<T> 
    where T: Iterator {

    pub (crate) fn new(iter: T) -> Self {
        Self {
            iter,
            lookahead: None,
        }
    }

    pub (crate) fn peek(&mut self) -> Option<&T::Item> {
        match &self.lookahead {
            Some(_) => self.lookahead.as_ref(),
            None => {
                self.lookahead = self.iter.next();
                self.lookahead.as_ref()
            }
        }
    }
}

impl<T> Iterator for MakePeekableIterator<T> 
    where T: Iterator {

    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::take(&mut self.lookahead) {
            Some(lookahead) => Some(lookahead),
            None => {
                self.iter.next()
            }
        }
    }
}