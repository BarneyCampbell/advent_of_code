open Printf
open Aoc.Functions
open Aoc.Files

let filepath= "data/day1.txt"

let next_bracket bracket n = match bracket with
  | '(' -> n+1
  | ')' -> n-1
  | _   -> n

let part1 = let file = read_string filepath |> to_char_list in
  fold next_bracket file 0

let rec find_basement brackets floor i = match brackets with
  | []      -> -1
  | ')'::[] -> i + 1
  | '('::[] -> -1
  | x::xs -> match x with
    | ')' -> if floor < 0 then i else find_basement xs (floor - 1) (i + 1)
    | '(' -> if floor < 0 then i else find_basement xs (floor + 1) (i + 1)
    | _   -> find_basement xs floor i

let part2 = let file = read_string filepath |> to_char_list in
  find_basement file 0 0


let () = 
  printf "Part 1: %d\n" part1 ;;
  printf "Part 2: %d\n" part2 ;;
