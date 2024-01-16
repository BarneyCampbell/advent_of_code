module Util where 

import System.IO
import Data.Maybe (fromMaybe)

fileCharArray :: String -> IO [String]
fileCharArray path = readFile path >>= return . lines

splitOnce :: String -> Char -> String
splitOnce []  _ = ""
splitOnce [x] _ = [x]
splitOnce (x:y:xs) c
    | y == c    = [x]
    | otherwise = x : splitOnce (y:xs) c

takeOne :: String -> Char -> (String, Maybe String)
takeOne []  _ = ("", Nothing)
takeOne [x] _ = ([x], Nothing)
takeOne (x:y:xs) c
    | y == c    = ([x], Just xs)
    | otherwise = bind x (str, maybeStr)
                  where (str, maybeStr) = takeOne (y:xs) c

splitMap :: (String -> a) -> String -> Char -> [a] 
splitMap f "" _  = []
splitMap f str c = do let (x, xs) = takeOne str c 
                      f x : splitMap f (fromMaybe "" xs) c

bind :: Char -> (String, Maybe String) -> (String, Maybe String)
bind c (str, rest) = (c:str, rest)

-- for easy composing
readInt :: String -> Int
readInt "" = 0
readInt str = read str :: Int
