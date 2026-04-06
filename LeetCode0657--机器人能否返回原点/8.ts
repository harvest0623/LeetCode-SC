function judgeCircle(moves: string): boolean {
    let x = 0, y = 0;
    const length = moves.length;
    
    for (let i = 0; i < length; i++) {
        switch (moves[i]) {
            case 'U':
                y--;
                break;
            case 'D':
                y++;
                break;
            case 'L':
                x--;
                break;
            case 'R':
                x++;
                break;
        }
    }
    
    return x === 0 && y === 0;
}