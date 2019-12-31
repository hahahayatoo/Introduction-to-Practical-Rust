pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capasity(0)
    }

    pub fn with_capasity(capasity: usize) -> Self {
        Self {
            elements: Self::allocation_in_heep(capasity),
            len: 0,
        }
    }

    fn allocation_in_heep(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capasity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capasity() {
            self.grow();
        }

        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    fn grow(&mut self) {
        if self.capasity() == 0 {
            self.elements = Self::allocation_in_heep(1);
        } else {
            let new_elements = Self::allocation_in_heep(self.capasity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);

            for (i, elm) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elm;
            }
        }
    } 

    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        // match self.get(index) {
        //     Some(v) => v,
        //     None => default,
        // }
        self.get(index).unwrap_or(default)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // let elm = self.elements[self.len];
            //           ^^^^^^^^^^^^^^^^^^^^^^^
            //           |
            //           cannot move out of borrowed content
            let elm = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elm)
        }
    }
}

impl<T: Default> ToyVec<T> {
    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }
}

pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>,
    len: usize,
    pos: usize,
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    type Item = &'vec T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
