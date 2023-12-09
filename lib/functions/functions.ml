let rec map f l = 
  match l with 
  | []      -> []
  | x::[]   -> (f x) :: []
  | x :: xs -> (f x) :: map f xs

let rec fold f l i =
  match l with
  | []    -> i
  | x::[] -> f x i
  | x::xs -> f x (fold f xs i)

let rec for_each f l = 
  match l with 
  | []      -> ()
  | x::[]   -> f x; ()
  | x :: xs -> f x; for_each f xs

let rec sum list = match list with
  | []      -> 0
  | x::[]   -> x
  | x::xs   -> x + sum xs

let to_char_list s = List.init (String.length s) (String.get s)