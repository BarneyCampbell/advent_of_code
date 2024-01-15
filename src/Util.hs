module Util where 

import System.IO

fileCharArray :: String -> IO [String]
fileCharArray path = readFile path >>= return . lines

readInt :: String -> Int
readInt str = read str :: Int

putAll :: [String] -> IO ()
putAll [] = return ()
putAll [x] = putStrLn x
putAll (x:xs) = do putStrLn x
                   putAll xs
