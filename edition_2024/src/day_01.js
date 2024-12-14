import { readFileSync } from "node:fs";
import { join } from "node:path";

const testInput = `3   4
4   3
2   5
1   3
3   9
3   3`;

/**
 *
 * @param {string} input the two space separated lists
 */
const solvePart1 = (input) => {
  const [list1, list2] = input.split("\n").reduce(
    (acc, line) => {
      const [elem1, elem2] = line.split(/ +/).map(Number);
      acc[0].push(elem1);
      acc[1].push(elem2);

      return acc;
    },
    [[], []]
  );

  const sorted1 = list1.sort((a, b) => a - b);
  const sorted2 = list2.sort((a, b) => a - b);

  let sum = 0;
  for (let i = 0; i < sorted1.length; i++) {
    const diff = sorted1[i] - sorted2[i];
    sum += Math.abs(diff);
  }

  return sum;
};

/**
 *
 * @param {string} input the two space separated lists
 */
const solvePart2 = (input) => {
  const [list1, map2] = input.split("\n").reduce(
    (acc, line) => {
      const [elem1, elem2] = line.split(/ +/).map(Number);
      acc[0].push(elem1);

      if (!acc[1].has(elem2)) {
        acc[1].set(elem2, 0);
      }
      acc[1].set(elem2, acc[1].get(elem2) + 1);

      return acc;
    },
    [[], new Map()]
  );

  return list1.reduce((acc, elem) => {
    const times = map2.get(elem) || 0;
    // console.log(acc, times);
    return acc + elem * times;
  }, 0);
};

console.log(solvePart1(testInput));
console.log(solvePart2(testInput));

const realInput = readFileSync(join(import.meta.dirname, "..", "res", "day_01.txt"), "utf-8");

console.log(solvePart1(realInput));
console.log(solvePart2(realInput));
