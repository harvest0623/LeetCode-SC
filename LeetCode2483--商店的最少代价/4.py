class Solution:
    def bestClosingTime(self, customers: str) -> int:
        n = len(customers)
        suf = 0 
        pre = 0
        min_cost = 0
        res = 0
        for i in range(n + 1):
            cost = suf + pre

            if min_cost > cost:
                min_cost = cost
                res = i

            if i < n:
                if customers[i] == 'N':
                    pre += 1
                else:
                    suf -= 1
        return res