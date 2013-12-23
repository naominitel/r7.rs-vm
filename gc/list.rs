// A simple linked-list the GC uses to keep track of allocated objects.
// extra::list wouldn't work since it uses @-ptrs and extra::Dlist doesn't
// allow random index removal

pub enum ListNode<T> {
    Empty,
    Node(T, ~ListNode<T>)
}

pub struct List<T> {
    head: ~ListNode<T>
}

impl<T> ListNode<T> {
    fn cons(~self, data: T) -> ~ListNode<T> {
        ~Node(data, self)
    }
}

impl<T> List<T> {
    pub fn new() -> ~List<T> {
        ~List { head: ~Empty }
    }

    pub fn insert(&mut self, t: T) {
        use std::util::swap;

        let mut nhead = ~Empty;
        swap(&mut self.head, &mut nhead);
        nhead = nhead.cons(t);
        swap(&mut self.head, &mut nhead);
    }
}
