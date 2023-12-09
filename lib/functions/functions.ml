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

let to_char_list s = List.init (String.length s) (String.get s)