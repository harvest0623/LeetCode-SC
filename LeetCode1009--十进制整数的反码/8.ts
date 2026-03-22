function bitwiseComplement(n: number): number {
    let sum = 1;
    while (sum < n) {
        sum = sum * 2 + 1;
    }
    return sum - n;
};