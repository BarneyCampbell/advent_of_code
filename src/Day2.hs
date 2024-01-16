module Day2 where 

import Computer
import Util ( readInt, takeOne, splitMap )
import Data.Maybe (fromMaybe)
import Data.Version (Version(versionBranch))

run :: String -> IO ()
run file = do input <- readFile file >>= (\inp -> return $ Util.splitMap Util.readInt inp ',')
              putStrLn "\nDay two:"
              putStr "Part 1: "
              print $ head . operate 0 dayTwoOps $ setVerbNoun 12 2 input
              putStr "Part 2: "
              let (noun, verb) = findBigNumber input
              print $ (100*noun) + verb

-- Finds the operations that leave 19690720
findBigNumber :: [Int] -> (Int, Int)
findBigNumber input = mapUntilEqual input 19690720 0 0

mapUntilEqual :: [Int] -> Int -> Int -> Int -> (Int, Int)
mapUntilEqual input search outer 100 = mapUntilEqual input search (outer+1) 0
mapUntilEqual _ _ 100 _ = (-1, 1)
mapUntilEqual input search outer inner
    | result == search = (outer, inner)
    | otherwise = mapUntilEqual input search outer (inner+1)
    where result = head . operate 0 dayTwoOps $ setVerbNoun outer inner input

dayTwoOps :: Int -> Int
dayTwoOps 1  = 4
dayTwoOps 2  = 4
dayTwoOps 99 = 1