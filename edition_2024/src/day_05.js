import { readFileSync } from "node:fs";
import { join } from "node:path";

const testInput = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`;

/**
 *
 * @param {string} input
 */
const solveDay5 = (input) => {
  // Split the input on the empty line
  const [rulesInput, updatesInput] = input.split("\n\n");

  const rules = rulesInput.split("\n").map((rule) => rule.split("|").map(Number));
  const updates = updatesInput.split("\n").map((update) => update.split(",").map(Number));

  const safeUnsafe = updates.reduce(
    (acc, update) => {
      if (isSafe(rules, update)) {
        acc[0].push(update);
      } else {
        const nowSafe = fixUnsafe(rules, update);
        // console.log(`Fixed ${update} to ${nowSafe}`);
        // console.log(isSafe(rules, nowSafe));
        acc[1].push(nowSafe);
      }

      return acc;
    },
    [[], []]
  );

  return safeUnsafe.map((updates) => sumMiddle(updates));
};

function sumMiddle(updates) {
  return updates.reduce((acc, update) => acc + update[Math.floor(update.length / 2)], 0);
}

/**
 * The input is safe if it obeys every rule. To obey a rule, it's first number needs to apper
 * before the second number, of neither of it's numbers appear in the input.
 *
 * @param {[string, string][]} rules The rules that need to be obeyed
 * @param {string[]} input The input to check
 * @returns {boolean} Whether the input is safe
 */
function isSafe(rules, input) {
  return rules.every(
    ([first, second]) =>
      input.indexOf(first) < input.indexOf(second) || input.indexOf(first) === -1 || input.indexOf(second) === -1
  );
}

/**
 * Fixes an unsafe input by swapping elements to obey the rules.
 *
 * @param {[number, number][]} rules - The rules that need to be obeyed.
 * @param {number[]} input - The input to fix.
 * @returns {number[]} The fixed input.
 */
function fixUnsafe(rules, input) {
  let fixed = [...input];

  for (const [first, second] of rules) {
    const indexOfFirst = fixed.indexOf(first);
    const indexOfSecond = fixed.indexOf(second);

    if (indexOfFirst === -1 || indexOfSecond === -1) {
      continue;
    }

    if (indexOfFirst > indexOfSecond) {
      const temp = fixed[indexOfFirst];
      fixed[indexOfFirst] = fixed[indexOfSecond];
      fixed[indexOfSecond] = temp;

      // By fixing a rule, we might have broken another one. So we need to check again.
      return fixUnsafe(rules, fixed);
    }
  }

  return fixed;
}

let [part1, part2] = solveDay5(testInput);
console.log(part1 === 143);
console.log(part2 === 123);

const realInput = readFileSync(join(import.meta.dirname, "..", "res", "day_05.txt"), "utf-8");

[part1, part2] = solveDay5(realInput);
console.log(part1 === 5964);
console.log(part2 === 4719);
