// 方法一：优先队列
var maxSlidingWindow = function(nums, k) {
    const n = nums.length;
    // 实现大顶堆（优先队列），存储 [数值, 索引] 对，按数值降序排列
    class MaxHeap {
        constructor() {
            this.heap = [];
        }

        // 插入元素
        push(val) {
            this.heap.push(val);
            this.bubbleUp(this.heap.length - 1);
        }

        // 弹出堆顶元素
        pop() {
            if (this.heap.length === 1) return this.heap.pop();
            const top = this.heap[0];
            this.heap[0] = this.heap.pop();
            this.bubbleDown(0);
            return top;
        }

        // 获取堆顶元素
        top() {
            return this.heap[0];
        }

        // 向上调整堆
        bubbleUp(index) {
            while (index > 0) {
                const parentIndex = (index - 1) >> 1;
                if (this.heap[parentIndex][0] >= this.heap[index][0]) break;
                [this.heap[parentIndex], this.heap[index]] = [this.heap[index], this.heap[parentIndex]];
                index = parentIndex;
            }
        }

        // 向下调整堆
        bubbleDown(index) {
            const length = this.heap.length;
            while (true) {
                let maxIndex = index;
                const leftChild = 2 * index + 1;
                const rightChild = 2 * index + 2;
                if (leftChild < length && this.heap[leftChild][0] > this.heap[maxIndex][0]) {
                    maxIndex = leftChild;
                }
                if (rightChild < length && this.heap[rightChild][0] > this.heap[maxIndex][0]) {
                    maxIndex = rightChild;
                }
                if (maxIndex === index) break;
                [this.heap[index], this.heap[maxIndex]] = [this.heap[maxIndex], this.heap[index]];
                index = maxIndex;
            }
        }
        // 判断堆是否为空
        isEmpty() {
            return this.heap.length === 0;
        }
    }
    const heap = new MaxHeap();
    // 初始化前k个元素入堆
    for (let i = 0; i < k; ++i) {
        heap.push([nums[i], i]);
    }
    const ans = [heap.top()[0]];
    // 滑动窗口向右移动
    for (let i = k; i < n; ++i) {
        // 新元素入堆
        heap.push([nums[i], i]);
        // 移除不在当前窗口内的堆顶元素（窗口范围：[i-k+1, i]）
        while (heap.top()[1] <= i - k) {
            heap.pop();
        }
        // 记录当前窗口最大值
        ans.push(heap.top()[0]);
    }
    return ans;
};

// 测试用例
console.log(maxSlidingWindow([1,3,-1,-3,5,3,6,7], 3)); // 输出: [3,3,5,5,6,7]
console.log(maxSlidingWindow([1], 1)); // 输出: [1]
console.log(maxSlidingWindow([1,-1], 1)); // 输出: [1,-1]

// 方法二：单调队列
var maxSlidingWindow = function(nums, k) {
    const n = nums.length;
    const q = [];
    for (let i = 0; i < k; i++) {
        while (q.length && nums[i] >= nums[q[q.length - 1]]) {
            q.pop();
        }
        q.push(i);
    }

    const ans = [nums[q[0]]];
    for (let i = k; i < n; i++) {
        while (q.length && nums[i] >= nums[q[q.length - 1]]) {
            q.pop();
        }
        q.push(i);
        while (q[0] <= i - k) {
            q.shift();
        }
        ans.push(nums[q[0]]);
    }
    return ans;
};