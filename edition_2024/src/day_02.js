import { readFileSync } from "node:fs";
import { join } from "node:path";

const testInput = `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`;

/**
 *
 * @param {string} sequence
 * @returns {boolean}
 */
function isSafe(sequence) {
  let direction = null;

  for (let i = 0; i < sequence.length - 1; i++) {
    const diff = sequence[i + 1] - sequence[i];
    const dir = diff > 0 ? "increasing" : "decreasing";
    if (diff === 0) {
      return false; // unsafe
    }

    if (direction === null) {
      direction = dir;
    } else if (direction !== dir) {
      return false; // unsafe
    }

    if (Math.abs(diff) > 3) {
      return false; // unsafe
    }
  }

  return true;
}

/**
 *
 * @param {string} input
 */
const solvePart1 = (input) => {
  return input.split("\n").reduce((acc, line) => {
    const sequence = line.split(/ +/).map(Number);

    return isSafe(sequence) ? acc + 1 : acc;
  }, 0);
};

/**
 *
 * @param {string} input
 */
const solvePart2 = (input) => {
  return input.split("\n").reduce((acc, line) => {
    const sequence = line.split(/ +/).map(Number);

    if (isSafe(sequence)) {
      return acc + 1;
    }

    return [...sequence.keys()].some((index) => isSafe(sequence.filter((_, i) => i !== index))) ? acc + 1 : acc;
  }, 0);
};

console.log(solvePart1(testInput));
console.log(solvePart2(testInput));

const realInput = readFileSync(join(import.meta.dirname, "..", "res", "day_02.txt"), "utf-8");

console.log(solvePart1(realInput));
console.log(solvePart2(realInput));
