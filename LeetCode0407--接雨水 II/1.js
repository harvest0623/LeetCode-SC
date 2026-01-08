// 方法一：最小堆
var trapRainWater = function(heightMap) {
    // 边界条件：行数或列数小于等于2时，无法接水
    if (heightMap.length <= 2 || heightMap[0].length <= 2) {
        return 0;
    }
    const m = heightMap.length;    // 行数
    const n = heightMap[0].length; // 列数
    // 优先级队列（最小堆）：JavaScript没有内置堆，这里手动实现最小堆
    class MinHeap {
        constructor() {
            this.heap = [];
        }
        // 插入元素
        push(val) {
            this.heap.push(val);
            this.bubbleUp(this.heap.length - 1);
        }
        // 弹出堆顶（最小值）
        pop() {
            if (this.heap.length === 1) {
                return this.heap.pop();
            }
            const top = this.heap[0];
            this.heap[0] = this.heap.pop();
            this.bubbleDown(0);
            return top;
        }
        // 获取堆顶元素
        top() {
            return this.heap[0];
        }
        // 判断堆是否为空
        isEmpty() {
            return this.heap.length === 0;
        }
        // 向上调整堆
        bubbleUp(index) {
            while (index > 0) {
                const parentIndex = Math.floor((index - 1) / 2);
                // 按高度升序排列（最小堆）
                if (this.heap[parentIndex][0] > this.heap[index][0]) {
                    [this.heap[parentIndex], this.heap[index]] = [this.heap[index], this.heap[parentIndex]];
                    index = parentIndex;
                } else {
                    break;
                }
            }
        }
        // 向下调整堆
        bubbleDown(index) {
            const lastIndex = this.heap.length - 1;
            while (true) {
                const leftChildIndex = index * 2 + 1;
                const rightChildIndex = index * 2 + 2;
                let smallestIndex = index;
                // 找到最小的子节点
                if (leftChildIndex <= lastIndex && this.heap[leftChildIndex][0] < this.heap[smallestIndex][0]) {
                    smallestIndex = leftChildIndex;
                }
                if (rightChildIndex <= lastIndex && this.heap[rightChildIndex][0] < this.heap[smallestIndex][0]) {
                    smallestIndex = rightChildIndex;
                }
                // 如果当前节点不是最小的，交换并继续调整
                if (smallestIndex !== index) {
                    [this.heap[index], this.heap[smallestIndex]] = [this.heap[smallestIndex], this.heap[index]];
                    index = smallestIndex;
                } else {
                    break;
                }
            }
        }
    }
    const pq = new MinHeap();
    // 访问标记数组：记录是否已处理过该位置
    const visit = Array.from({ length: m }, () => Array(n).fill(false));
    // 初始化：将边界的格子加入优先级队列，并标记为已访问
    for (let i = 0; i < m; i++) {
        for (let j = 0; j < n; j++) {
            if (i === 0 || i === m - 1 || j === 0 || j === n - 1) {
                pq.push([heightMap[i][j], i * n + j]); // [高度, 一维索引]
                visit[i][j] = true;
            }
        }
    }
    let res = 0; // 总接水量
    // 四个方向：上、右、下、左
    const dirs = [-1, 0, 1, 0, -1];
    // 处理优先级队列中的元素
    while (!pq.isEmpty()) {
        const curr = pq.top();
        pq.pop();
        const currHeight = curr[0];
        const currPos = curr[1];
        const x = Math.floor(currPos / n); // 还原行索引
        const y = currPos % n;            // 还原列索引

        // 遍历四个方向的相邻格子
        for (let k = 0; k < 4; k++) {
            const nx = x + dirs[k];
            const ny = y + dirs[k + 1];
            // 检查相邻格子是否在范围内且未被访问
            if (nx >= 0 && nx < m && ny >= 0 && ny < n && !visit[nx][ny]) {
                // 如果相邻格子高度小于当前格子高度，能接水
                if (heightMap[nx][ny] < currHeight) {
                    res += currHeight - heightMap[nx][ny];
                }
                // 标记为已访问，并将该格子加入队列（高度取最大值）
                visit[nx][ny] = true;
                pq.push([Math.max(heightMap[nx][ny], currHeight), nx * n + ny]);
            }
        }
    }
    return res;
};

// 测试用例
// 示例1：输入 [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]，输出 4
console.log(trapRainWater([[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]])); 
// 示例2：输入 [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]，输出 10
console.log(trapRainWater([[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]));

// 方法二：广度优先搜索
var trapRainWater = function(heightMap) {
    const m = heightMap.length;
    const n = heightMap[0].length;
    const dirs = [-1, 0, 1, 0, -1];
    let maxHeight = 0;
    
    for (let i = 0; i < m; ++i) {
        for (let j = 0; j < n; ++j) {
            maxHeight = Math.max(maxHeight, heightMap[i][j]);
        }
    }
    const water = new Array(m).fill(0).map(() => new Array(n).fill(0));
    for (let i = 0; i < m; ++i) {
        for (let j = 0; j < n; ++j){
            water[i][j] = maxHeight;      
        }
    }  
    const qu = [];
    for (let i = 0; i < m; ++i) {
        for (let j = 0; j < n; ++j) {
            if (i == 0 || i == m - 1 || j == 0 || j == n - 1) {
                if (water[i][j] > heightMap[i][j]) {
                    water[i][j] = heightMap[i][j];
                    qu.push([i, j]);
                }
            }
        }
    } 
    while (qu.length) {
        const curr = qu.shift();
        const x = curr[0];
        const y = curr[1];
        for (let i = 0; i < 4; ++i) {
            const nx = x + dirs[i], ny = y + dirs[i + 1];
            if (nx < 0 || nx >= m || ny < 0 || ny >= n) {
                continue;
            }
            if (water[x][y] < water[nx][ny] && water[nx][ny] > heightMap[nx][ny]) {
                water[nx][ny] = Math.max(water[x][y], heightMap[nx][ny]);
                qu.push([nx, ny]);
            }
        }
    }

    let res = 0;
    for (let i = 0; i < m; ++i) {
        for (let j = 0; j < n; ++j) {
            res += water[i][j] - heightMap[i][j];
        }
    }
    return res;
};