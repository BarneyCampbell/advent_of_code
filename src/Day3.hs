module Day3 where

import qualified Data.Set (Set)

import Util (fileCharArray, takeOne, readInt)
import Data.Maybe (isNothing)
import qualified Data.Set as Set

run :: String -> IO ()
run path = do lines <- fileCharArray path
              --print . head $ lines
              let path1 = plotPath (Just $ head lines) (Coordinate 0 0)
              let path2 = plotPath (Just $ lines !! 1) (Coordinate 0 0)
              print $ minimum $ Set.map manhattanOrigin (getIntersection path1 path2)

manhattanOrigin :: Coordinate Int Int -> Int
manhattanOrigin (Coordinate x y) = abs x + abs y

getIntersection :: [Coordinate Int Int] -> [Coordinate Int Int] -> Data.Set.Set (Coordinate Int Int)
getIntersection l r = Set.intersection (Set.fromList l) (Set.fromList r)

plotPath :: Maybe String -> Coordinate Int Int -> [Coordinate Int Int]
plotPath Nothing _ = []
plotPath (Just path) position
    | null nextMove = []
    | otherwise = expandDifference (unwrapMove nextMove) position <> plotPath rest (moveToCoord nextMove position)
    where (nextMove, rest) = takeOne path ','

expandDifference ::(Direction, Int) -> Coordinate Int Int -> [Coordinate Int Int]
expandDifference (_, 0) _ = []
expandDifference (Up, target) cur = next : expandDifference (Up, target-1) next
                                 where next = addCoords cur coordUp
expandDifference (Down, target) cur = next : expandDifference (Down, target-1) next
                                 where next = addCoords cur coordDown
expandDifference (Day3.Left, target) cur = next : expandDifference (Day3.Left, target-1) next
                                 where next = addCoords cur coordLeft
expandDifference (Day3.Right, target) cur = next : expandDifference (Day3.Right, target-1) next
                                 where next = addCoords cur coordRight

unwrapMove :: String -> (Direction, Int)
unwrapMove ('U':num) = (Up, readInt num)
unwrapMove ('D':num) = (Down, readInt num)
unwrapMove ('L':num) = (Day3.Left, readInt num)
unwrapMove ('R':num) = (Day3.Right, readInt num)

moveToCoord :: String -> Coordinate Int Int -> Coordinate Int Int
moveToCoord ('U':num) = addCoords $ Coordinate 0 (readInt num)
moveToCoord ('D':num) = addCoords $ Coordinate 0 (-1 * readInt num)
moveToCoord ('L':num) = addCoords $ Coordinate (-1 * readInt num) 0
moveToCoord ('R':num) = addCoords $ Coordinate (readInt num) 0

data Coordinate x y = Coordinate x y
    deriving (Eq, Ord, Show)

data Direction = Up | Down | Left | Right

addCoords :: (Num x, Num y) => Coordinate x y -> Coordinate x y -> Coordinate x y
addCoords (Coordinate x1 y1) (Coordinate x2 y2) = Coordinate (x1+x2) (y1+y2)

setFirst :: Coordinate x y -> x -> Coordinate x y
setFirst (Coordinate _ y) new = Coordinate new y 

setSecond :: Coordinate x y -> y -> Coordinate x y
setSecond (Coordinate x _) = Coordinate x

coordUp = Coordinate 0 1
coordDown = Coordinate 0 (-1)
coordLeft = Coordinate (-1) 0
coordRight = Coordinate 1 0