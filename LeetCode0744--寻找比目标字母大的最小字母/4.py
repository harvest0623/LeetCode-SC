class Solution:
    def nextGreatestLetter(self, letters: List[str], target: str) -> str:
        return next((letter for letter in letters if letter > target), letters[0])