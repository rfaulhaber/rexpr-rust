use std::vec::Vec;

pub struct Stack<T> {
    v: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        let v: Vec<T> = Vec::new();
        Stack { v }
    }

    pub fn push(&mut self, t: T) {
        self.v.push(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_adds_to_stack() {
        let mut s = Stack::new();

        s.push(String::from("first"));

        assert_eq!(s.v[0], "first");
    }

    #[test]
    fn pop_removes_last_element_from_stack() {
        let mut s = Stack::new();

        s.push(String::from("first"));
        s.push(String::from("second"));

        let ret = s.pop();

        assert_eq!(ret.is_some(), true);
        assert_eq!(ret.unwrap(), "second");
    }

    #[test]
    fn len_returns_length() {
        let mut s = Stack::new();

        s.push(String::from("first"));
        s.push(String::from("second"));

        let length = s.len();

        assert_eq!(length, 2);
        assert_eq!(s.v.len(), s.len());
    }
}
