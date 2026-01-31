var Bank = function(balance) {
    this.b = balance;
};

Bank.prototype.transfer = function(account1, account2, money) {
    if (account1 > this.b.length || account2 > this.b.length || this.b[account1 - 1] < money) {
        return false;
    }
    this.b[account1 - 1] -= money;
    this.b[account2 - 1] += money;
    return true;
};

Bank.prototype.deposit = function(account, money) {
    if (account > this.b.length) {
        return false;
    }
    this.b[account - 1] += money;
    return true;
};

Bank.prototype.withdraw = function(account, money) {
    if (account > this.b.length || this.b[account - 1] < money) {
        return false;
    }
    this.b[account - 1] -= money;
    return true;
};