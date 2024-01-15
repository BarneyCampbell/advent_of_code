module Main where

import qualified Day1 (run)
import qualified Day2 (otherFunc)

import Util

main :: IO ()
main = do
  Day1.run "data/day1.txt"
