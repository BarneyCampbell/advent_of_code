open Aoc.Files
open Aoc.Functions
open Printf

let filepath = "data/day2.txt"

let min_side x y z = 
  min (x * y) (min (x * z) (y * z))
;;

let get_dims str = let nums = map int_of_string (String.split_on_char 'x' str) in
  match nums with
  | []          -> 0
  | _::[]       -> 0
  | _::_::[]    -> 0
  | l::w::h::_  -> (2*l*w + 2*w*h + 2*h*l) + min_side l w h
;;

let part1 = let file = read_lines filepath in
  sum (map get_dims file)
;;

let min_perimeter x y z = 
  min (2*x + 2*y) (min (2*x + 2*z) (2*y + 2*z))
;;

let get_ribbon str = let nums = map int_of_string (String.split_on_char 'x' str) in
  match nums with
  | []          -> 0
  | _::[]       -> 0
  | _::_::[]    -> 0
  | l::w::h::_  -> (l * w * h) + min_perimeter l w h
;;

let part2 = let file = read_lines filepath in 
  sum (map get_ribbon file)
;;


let () = 
  printf "Part 1: %d\n" part1 ;;
  printf "Part 2: %d\n" part2 ;;
