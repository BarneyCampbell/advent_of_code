module Day1 where

import Util
import Numeric (readInt)

run :: String -> IO ()
run file = do lines <- Util.fileCharArray file >>= mapM (return . Util.readInt) -- Crazy
              putStrLn "\nDay one:"
              putStr "Part 1: "
              print . sum $ map getFuel lines
              putStr "Part 2: "
              print . sum $ map getFuelWithFuelFuel lines

getFuel :: Int -> Int
getFuel val = div val 3 - 2 -- Cursed

getFuelWithFuelFuel :: Int -> Int
getFuelWithFuelFuel val
    | val < 0   = 0
    | otherwise = if fuel >= 0 then fuel + getFuelWithFuelFuel fuel
                     else 0
                  where fuel = getFuel val
