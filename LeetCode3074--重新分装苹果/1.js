var minimumBoxes = function(apple, capacity) {
    let s = _.sum(apple); // 把所有苹果堆在一起
    capacity.sort((a, b) => b - a); // 先装大箱子，再装小箱子
    let i = 0;
    while (s > 0) { // 还有剩余苹果
        s -= capacity[i++]; // 使用一个箱子
    }
    return i;
};