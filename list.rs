enum ListNode<T> {
    Empty,
    Node(T, ~ListNode<T>)
}

struct List<T> {
    head: ~ListNode<T>
}

impl<T> ListNode<T> {
    fn cons(~self, data: T) -> ~ListNode<T> {
        ~Node(data, self)
    }
}

impl<T> List<T> {
    pub fn insert(&mut self, t: T) {
        use std::util::swap;

        let mut nhead = ~Empty;
        swap(&mut self.head, &mut nhead);
        nhead = nhead.cons(t);
        swap(&mut self.head, &mut nhead);
    }
}
