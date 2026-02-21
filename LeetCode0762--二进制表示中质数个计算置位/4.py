class Solution:
    def isPrime(self, x: int) -> bool:
        if x < 2:
            return False
        i = 2
        while i * i <= x:
            if x % i == 0:
                return False
            i += 1
        return True

    def countPrimeSetBits(self, left: int, right: int) -> int:
        return sum(self.isPrime(x.bit_count()) for x in range(left, right + 1))