class Solution:
    def getHappyString(self, n: int, k: int) -> str:
        chs = ['a', 'b', 'c']
        res = []
        if k > 3 * (1 << (n - 1)):
            return "".join(res)
        for i in range(n):
            if len(res) != i:
                break
            count = 1 << (n - i - 1)
            for c in chs:
                if res and res[-1] == c:
                    continue
                if k <= count:
                    res.append(c)
                    break
                k -= count
        return "".join(res)