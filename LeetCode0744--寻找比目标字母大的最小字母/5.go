func nextGreatestLetter(letters []byte, target byte) byte {
    for _, letter := range letters {
        if letter > target {
            return letter
        }
    }
    return letters[0]
}