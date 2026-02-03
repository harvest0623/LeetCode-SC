func countCollisions(s string) int {
	s = strings.TrimLeft(s, "L")          // 前缀向左的车不会发生碰撞
	s = strings.TrimRight(s, "R")         // 后缀向右的车不会发生碰撞
	return len(s) - strings.Count(s, "S") // 剩下非静止的车计入碰撞次数
}