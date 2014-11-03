// A simple linked-list the GC uses to keep track of allocated objects.
// extra::list wouldn't work since it uses @-ptrs and extra::Dlist doesn't
// allow random index removal

pub enum ListNode<T> {
    Empty,
    Node(T, Box<ListNode<T>>)
}

pub struct List<T> {
    pub head: Box<ListNode<T>>
}

impl<T> ListNode<T> {
    fn cons(self, data: T) -> Box<ListNode<T>> {
        box Node(data, box self)
    }
}

impl<T> List<T> {
    pub fn new() -> Box<List<T>> {
        box List { head: box Empty }
    }

    pub fn insert(&mut self, t: T) {
        use std::mem::swap;

        let mut nhead = box Empty;
        swap(&mut self.head, &mut nhead);
        nhead = nhead.cons(t);
        swap(&mut self.head, &mut nhead);
    }
}
