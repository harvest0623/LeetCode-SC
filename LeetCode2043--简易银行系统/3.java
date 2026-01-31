class Bank {
    private final long[] b;

    public Bank(long[] balance) {
        b = balance;
    }

    public boolean transfer(int account1, int account2, long money) {
        if (account1 > b.length || account2 > b.length || b[account1 - 1] < money) {
            return false;
        }
        b[account1 - 1] -= money;
        b[account2 - 1] += money;
        return true;
    }

    public boolean deposit(int account, long money) {
        if (account > b.length) {
            return false;
        }
        b[account - 1] += money;
        return true;
    }

    public boolean withdraw(int account, long money) {
        if (account > b.length || b[account - 1] < money) {
            return false;
        }
        b[account - 1] -= money;
        return true;
    }
}