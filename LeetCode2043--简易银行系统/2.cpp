class Bank {
    vector<long long> b;

public:
    Bank(vector<long long>& balance) : b(balance) {}

    bool transfer(int account1, int account2, long long money) {
        if (account1 > b.size() || account2 > b.size() || b[account1 - 1] < money) {
            return false;
        }
        b[account1 - 1] -= money;
        b[account2 - 1] += money;
        return true;
    }

    bool deposit(int account, long long money) {
        if (account > b.size()) {
            return false;
        }
        b[account - 1] += money;
        return true;
    }

    bool withdraw(int account, long long money) {
        if (account > b.size() || b[account - 1] < money) {
            return false;
        }
        b[account - 1] -= money;
        return true;
    }
};