import { readFileSync } from "node:fs";
import { join } from "node:path";

const testInput = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

/**
 *
 * @param {string} input
 */
const solvePart1 = (input) => {
  const regex = /mul\((\d{1,3}),(\d{1,3})\)/g;

  const result = input.matchAll(regex);
  return [...result].reduce((acc, cur) => {
    return acc + Number(cur[1]) * Number(cur[2]);
  }, 0);
};

/**
 *
 * @param {string} input
 */
const solvePart2 = (input) => {
  const regex = /(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)/g;

  const result = input.matchAll(regex);
  return [...result].reduce(
    (acc, cur) => {
      if (cur[4]) {
        return [true, acc[1]];
      }
      if (cur[5]) {
        return [false, acc[1]];
      }

      if (acc[0]) {
        return [true, acc[1] + Number(cur[2]) * Number(cur[3])];
      } else {
        return acc;
      }
    },
    [true, 0]
  );
};

console.log(solvePart1(testInput));
console.log(solvePart2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"));

const realInput = readFileSync(join(import.meta.dirname, "..", "res", "day_03.txt"), "utf-8");

console.log(solvePart1(realInput));
console.log(solvePart2(realInput));
