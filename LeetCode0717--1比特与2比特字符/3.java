// 方法一：正序遍历
class Solution {
    public boolean isOneBitCharacter(int[] bits) {
        int n = bits.length, i = 0;
        while (i < n - 1) {
            i += bits[i] + 1;
        }
        return i == n - 1;
    }
}

// 方法二：倒序遍历
class Solution {
    public boolean isOneBitCharacter(int[] bits) {
        int n = bits.length, i = n - 2;
        while (i >= 0 && bits[i] == 1) {
            i--;
        }
        return (n - i) % 2 == 0;
    }
}
