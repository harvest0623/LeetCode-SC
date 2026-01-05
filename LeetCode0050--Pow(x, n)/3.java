class Solution {
    public double myPow(double x, int n) {
        if (n < 0) {
            return pow(1 / x, -(long) n);
        }
        return pow(x, n);
    }
    private double pow(double x, long n) {
        if (n == 0) {
            return 1;
        }
        double res = pow(x, n / 2);
        res *= res;
        if (n % 2 != 0) {
            res *= x;
        }
        return res;
    }
}