var findAllPeople = function (n, meetings, firstPerson) {
    meetings.sort((a, b) => a[2] - b[2]);
    const know = new Array(n).fill(false);
    know[0] = true;
    know[firstPerson] = true;
    let i = 0;
    while (i < meetings.length) {
        const time = meetings[i][2];
        const parent = new Map();
        const find = (x) => {
            if (!parent.has(x)) parent.set(x, x);
            if (parent.get(x) !== x) {
                parent.set(x, find(parent.get(x)));
            }
            return parent.get(x);
        };
        const union = (a, b) => {
            const pa = find(a);
            const pb = find(b);
            if (pa !== pb) parent.set(pb, pa);
        };
        let j = i;
        while (j < meetings.length && meetings[j][2] === time) {
            union(meetings[j][0], meetings[j][1]);
            j++;
        }
        const hasSecret = new Map();
        for (const x of parent.keys()) {
            const root = find(x);
            if (know[x]) hasSecret.set(root, true);
        }
        for (const x of parent.keys()) {
            if (hasSecret.get(find(x))) {
                know[x] = true;
            }
        }
        i = j;
    }
    const res = [];
    for (let i = 0; i < n; i++) {
        if (know[i]) res.push(i);
    }
    return res;
};