module Computer where
import Control.Exception (throw)

operate :: Int -> (Int -> Int) -> [Int] -> [Int]
operate index ops arr
    | operation == [99] = arr
    | null operation    = arr
    | otherwise         = operate (index + length operation) ops (applyOp operation arr)
    where operation = getOps index (ops $ arr !! index) arr

applyOp :: [Int] -> [Int] -> [Int]
applyOp [1,p1,p2,dest] arr = alterIndex dest ((arr !! p1) + (arr !! p2)) arr
applyOp [2,p1,p2,dest] arr = alterIndex dest ((arr !! p1) * (arr !! p2)) arr
applyOp ops arr = error $ "Unknown operation applied:  " ++ show ops ++ " on " ++ show arr

alterIndex :: Int -> Int -> [Int] -> [Int]
alterIndex 0 val (x:xs)       = val:xs
alterIndex cur val (x:xs) = x : alterIndex (cur-1) val xs

getOps :: Int -> Int -> [Int] -> [Int]
getOps _ 0 _           = []
getOps 0 count (x:xs)     = x : getOps 0 (count - 1) xs 
getOps index count (x:xs) = getOps (index-1) count xs

setVerbNoun :: Int -> Int -> [Int] -> [Int]
setVerbNoun noun verb = alterIndex 2 verb . alterIndex 1 noun

-- More of a guide for how to implement the ops argument
standardOps :: Int -> Int
standardOps 1  = 4
standardOps 2  = 4
standardOps 99 = 1