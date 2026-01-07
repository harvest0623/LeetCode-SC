class MyQueue:

    def __init__(self):
        self.A, self.B = [], []

    def push(self, x: int) -> None:
        self.A.append(x)

    def pop(self) -> int:
        peek = self.peek()
        self.B.pop()
        return peek

    def peek(self) -> int:
        if self.B: return self.B[-1]
        if not self.A: return -1
        # 将栈 A 的元素依次移动至栈 B
        while self.A:
            self.B.append(self.A.pop())
        return self.B[-1]

    def empty(self) -> bool:
        return not self.A and not self.B