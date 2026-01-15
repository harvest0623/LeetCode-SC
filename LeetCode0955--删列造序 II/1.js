var minDeletionSize = function(A) {
    const cuts = new Array(A.length - 1).fill(false);
    let ans = 0;
    for (let j = 0; j < A[0].length; j++) {
        let canKeep = true;
        for (let i = 0; i < A.length - 1; i++) {
            if (!cuts[i] && A[i][j] > A[i + 1][j]) {
                canKeep = false;
                break;
            }
        }
        if (canKeep) {
            for (let i = 0; i < A.length - 1; i++) {
                if (A[i][j] < A[i + 1][j]) {
                    cuts[i] = true;
                }
            }
        } else {
            ans++;
        }
    }
    return ans;
};