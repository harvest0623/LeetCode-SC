func readBinaryWatch(turnedOn int) (ans []string) {
    for h := range 12 {
        for m := range 60 {
            if bits.OnesCount8(uint8(h))+bits.OnesCount8(uint8(m)) == turnedOn {
                ans = append(ans, fmt.Sprintf("%d:%02d", h, m))
            }
        }
    }
    return
}