import { readFileSync } from "node:fs";
import { join } from "node:path";

const testInput = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`;

/**
 *
 * @param {string} input
 */
const solvePart1 = (input) => {
  const [diagonal1, diagonal2] = getDiagonalRepresentations(input);
  return countBasic(input) + countBasic(transpose(input)) + countBasic(diagonal1) + countBasic(diagonal2);
};

/**
 *
 * @param {string} input
 */
const solvePart2 = (input) => {
  const lines = input.split("\n");
  let total = 0;

  for (let i = 0; i < lines.length - 2; i++) {
    for (let j = 0; j < lines[0].length - 2; j++) {
      const hasMiddleA = lines[i + 1][j + 1] === "A";

      if (!hasMiddleA) {
        continue;
      }

      const letter1 = lines[i][j];
      const letter2 = lines[i + 2][j + 2];

      const letter3 = lines[i][j + 2];
      const letter4 = lines[i + 2][j];

      if (letter1 === "M" && letter2 == "S") {
        if ((letter3 === "M" && letter4 === "S") || (letter3 === "S" && letter4 === "M")) {
          total++;
        }
      } else if (letter1 === "S" && letter2 === "M") {
        if ((letter3 === "M" && letter4 === "S") || (letter3 === "S" && letter4 === "M")) {
          total++;
        }
      }
    }
  }

  return total;
};

/**
 * Transposes the text as if it were a matrix
 * @param {string} text
 * @returns {string}
 */
function transpose(text) {
  const lines = text.split("\n");
  const transposedInit = Array(lines[0].length).fill("");

  return lines
    .reduce((transposed, line) => {
      line.split("").forEach((char, cIndex) => {
        transposed[cIndex] += char;
      });

      return transposed;
    }, transposedInit)
    .join("\n");
}

/**
 * Counts the number of XMAS and SAMX in the text
 * @param {string} text
 * @returns {number}
 */
function countBasic(text) {
  const xmasCount = (text.match(/XMAS/g) || []).length;
  const samxCount = (text.match(/SAMX/g) || []).length;

  return xmasCount + samxCount;
}

/**
 * Returns the diagonals of the input
 * @param {string} input
 * @returns {[string, string]}
 */
function getDiagonalRepresentations(input) {
  const rows = input.split("\n");
  const n = rows.length;
  const m = rows[0].length;

  const diagonals1 = Array(n + m - 1).fill("");
  const diagonals2 = Array(n + m - 1).fill("");

  for (let i = 0; i < n; i++) {
    for (let j = 0; j < m; j++) {
      diagonals1[i + j] += rows[i][j];
    }
  }

  for (let i = 0; i < n; i++) {
    for (let j = 0; j < m; j++) {
      diagonals2[j - i + (n - 1)] += rows[i][j];
    }
  }

  const diagonal1Result = diagonals1.join("\n");
  const diagonal2Result = diagonals2.join("\n");

  return [diagonal1Result, diagonal2Result];
}

console.log(solvePart1(testInput));
console.log(solvePart2(testInput));

const realInput = readFileSync(join(import.meta.dirname, "..", "res", "day_04.txt"), "utf-8");

console.log(solvePart1(realInput));
console.log(solvePart2(realInput));
