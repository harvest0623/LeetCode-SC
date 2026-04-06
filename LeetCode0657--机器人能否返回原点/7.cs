public class Solution {
    public bool JudgeCircle(string moves) {
        int x = 0, y = 0;
        int length = moves.Length;
        
        for (int i = 0; i < length; i++) {
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
        
        return x == 0 && y == 0;
    }
}