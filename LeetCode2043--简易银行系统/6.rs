struct Bank {
    b: Vec<i64>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { b: balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let account1 = account1 as usize - 1;
        let account2 = account2 as usize - 1;
        if account1 >= self.b.len() || account2 >= self.b.len() || self.b[account1] < money {
            return false;
        }
        self.b[account1] -= money;
        self.b[account2] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize - 1;
        if account >= self.b.len() {
            return false;
        }
        self.b[account] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize - 1;
        if account >= self.b.len() || self.b[account] < money {
            return false;
        }
        self.b[account] -= money;
        true
    }
}