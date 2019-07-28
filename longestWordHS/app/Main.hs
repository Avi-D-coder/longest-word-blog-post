module Main where

import           Data.List
import           Data.Ord
import           Lib

isLetterAllowed :: Char -> Bool
isLetterAllowed letter =
  letter `notElem` ['g', 'k', 'm', 'q', 'v', 'w', 'x', 'z']

isWordAllowed :: String -> Bool
isWordAllowed = all isLetterAllowed

longestInList =
  maximumBy (\largest word -> length largest `compare` length word)

-- slower almost by 2x blog about it
longestWordSort = readFile "./words.txt" >>= \dictStr ->
  print $ head $ sortOn (Down . length) $ filter isWordAllowed $ lines dictStr

longestWord = readFile "./words.txt"
  >>= \dictStr -> print $ longestInList $ filter isWordAllowed $ lines dictStr

main = longestWord
