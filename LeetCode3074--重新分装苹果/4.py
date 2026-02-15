class Solution:
    def minimumBoxes(self, apple: List[int], capacity: List[int]) -> int:
        s = sum(apple)  
        capacity.sort(reverse=True) 
        for i, x in enumerate(capacity):
            s -= x
            if s <= 0:  
                return i + 1 