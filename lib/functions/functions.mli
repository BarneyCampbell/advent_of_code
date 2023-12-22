val map : ('a -> 'b) -> 'a list -> 'b list
val fold : ('a -> 'b -> 'b) -> 'a list -> 'b -> 'b
val for_each : ('a -> unit) -> 'a list -> unit

val sum : int list -> int

val to_char_list : string -> char list
