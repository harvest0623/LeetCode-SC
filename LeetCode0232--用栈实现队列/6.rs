struct MyQueue {
    ins: Vec<i32>,
    out: Vec<i32>,
}   

impl MyQueue {
    fn new() -> Self {
        Self {
            ins: vec![],
            out: vec![],
        }
    }

    fn tansfer(&mut self) {
        while let Some(x) = self.ins.pop() {
            self.out.push(x);
        }
    }

    fn push(&mut self, x: i32) {
        self.ins.push(x);
    }

    fn pop(&mut self) -> i32 {
        if (self.out.is_empty()) {
            self.tansfer();
        }

        self.out.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if (self.out.is_empty()) {
            self.tansfer();
        }

        *self.out.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.out.is_empty() && self.ins.is_empty()
    }
}