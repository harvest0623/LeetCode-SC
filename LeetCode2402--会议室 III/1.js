function mostBooked(n, meetings) {
    meetings.sort((a, b) => a[0] - b[0]);
    const availRooms = new MinPriorityQueue({ compare: (a, b) => a - b });
    for (let i = 0; i < n; i++) {
        availRooms.enqueue(i);
    }
    const usedRooms = new MinPriorityQueue({
        compare: (a, b) => a[0] === b[0] ? a[1] - b[1] : a[0] - b[0]
    });
    const usedCount = Array(n).fill(0);
    let curTime = 0;
    for (const [start, end] of meetings) {
        curTime = Math.max(curTime, start);
        while (!usedRooms.isEmpty() && usedRooms.front()[0] <= curTime) {
            availRooms.enqueue(usedRooms.dequeue()[1]);
        }
        if (availRooms.isEmpty()) {
            curTime = usedRooms.front()[0];
            while (!usedRooms.isEmpty() && usedRooms.front()[0] <= curTime) {
                availRooms.enqueue(usedRooms.dequeue()[1]);
            }
        }
        const room = availRooms.dequeue();
        usedCount[room]++;
        usedRooms.enqueue([curTime + end - start, room]);
    }
    let ans = 0;
    for (let i = 1; i < n; i++) {
        if (usedCount[i] > usedCount[ans]) {
            ans = i;
        }
    }
    return ans;
}