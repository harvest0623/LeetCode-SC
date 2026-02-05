var successfulPairs = function(spells, potions, success) {
    potions.sort((a, b) => a - b);
    for (let i = 0; i < spells.length; i++) {
        const target = success / spells[i];
        spells[i] = potions.length - _.sortedIndex(potions, target);
    }
    return spells;
};