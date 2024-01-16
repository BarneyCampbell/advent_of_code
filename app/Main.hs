module Main where

import qualified Day1 (run)
import qualified Day2 (run)
import qualified Day3 (run)

import Util

-- Todo args
main :: IO ()
main = do
  -- Day1.run "data/day1.txt"
  -- Day2.run "data/day2.txt"
  Day3.run "data/day3.txt"
