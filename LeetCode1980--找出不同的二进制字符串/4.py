class Solution:
    def findDifferentBinaryString(self, nums: List[str]) -> str:
        n = len(nums)
        # 预处理对应整数的哈希集合
        vals = {int(num, 2) for num in nums}
        # 寻找第一个不在哈希集合中的整数
        val = 0
        while val in vals:
            val += 1
        # 将整数转化为二进制字符串返回
        res = "{:b}".format(val)
        return '0' * (n - len(res)) + res