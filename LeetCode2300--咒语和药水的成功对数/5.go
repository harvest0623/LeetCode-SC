func successfulPairs(spells, potions []int, success int64) []int {
    slices.Sort(potions)
    for i, x := range spells {
        target := float64(success) / float64(x)
        j := sort.Search(len(potions), func(j int) bool { return float64(potions[j]) >= target })
        spells[i] = len(potions) - j
    }
    return spells
}