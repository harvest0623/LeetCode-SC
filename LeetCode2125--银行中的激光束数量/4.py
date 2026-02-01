class Solution:
    def numberOfBeams(self, bank: List[str]) -> int:
        last = ans = 0
        for line in bank:
            cnt = line.count("1")
            if cnt != 0:
                ans += last * cnt
                last = cnt
        return ans