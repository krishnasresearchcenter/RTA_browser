impl BinaryTreeNode {
    fn rotate_left(&mut self) {
        // Assume `self` is x in the x -> y rotation
        let mut y = self.right.take().expect("Right child must exist for left rotation");
        self.right = y.borrow_mut().left.take();

        if let Some(right) = &self.right {
            right.borrow_mut().parent = Rc::downgrade(&Rc::new(RefCell::new(self.clone())));
        }

        y.borrow_mut().left = Some(Rc::new(RefCell::new(self.clone())));
        y.borrow_mut().parent = self.parent.clone();
        self.parent = Rc::downgrade(&Rc::new(RefCell::new(y.borrow().clone())));

        // Update root if needed: This part depends on your tree structure and should be adapted accordingly.
    }

    fn rotate_right(&mut self) {
        // Assume `self` is y in the x <- y rotation
        let mut x = self.left.take().expect("Left child must exist for right rotation");
        self.left = x.borrow_mut().right.take();

        if let Some(left) = &self.left {
            left.borrow_mut().parent = Rc::downgrade(&Rc::new(RefCell::new(self.clone())));
        }

        x.borrow_mut().right = Some(Rc::new(RefCell::new(self.clone())));
        x.borrow_mut().parent = self.parent.clone();
        self.parent = Rc::downgrade(&Rc::new(RefCell::new(x.borrow().clone())));

        // Update root if needed: This part depends on your tree structure and should be adapted accordingly.
    }
}
