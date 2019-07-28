"use strict";
const fs = require("fs");

const BANNED_LETTERS = ["g", "k", "m", "q", "v", "w", "x", "z"];

const words = fs
  .readFileSync("./words.txt")
  .toString()
  .split("\n");

// function isLetterAllowed(word) {
//   BANNED_LETTERS.every(bannedLetter => bannedLetter != letter);
// }

const isLetterAllowed = letter =>
  BANNED_LETTERS.every(bannedLetter => bannedLetter != letter);

const isWordAllowed = word =>
  word.split("").every(letter => isLetterAllowed(letter));

const longerWord = (largest, word) =>
  largest.length > word.length ? largest : word;

const longestWord = words.filter(isWordAllowed).reduce(longerWord);

console.log(longestWord);
