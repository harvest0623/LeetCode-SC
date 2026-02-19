class Solution:
    def maximumTotalDamage(self, power: List[int]) -> int:
        cnt = Counter(power)
        a = sorted(cnt)
        f = [0] * (len(a) + 1)
        j = 0
        for i, x in enumerate(a):
            while a[j] < x - 2:
                j += 1
            f[i + 1] = max(f[i], f[j] + x * cnt[x])
        return f[-1]