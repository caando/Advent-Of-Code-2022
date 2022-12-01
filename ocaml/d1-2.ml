let max (a, b, c) d =
  if d > a then (d, a, b)
  else if d > b then (a, d, b)
    else if d > c then (a, b, d)
    else (a, b, c);; 

let rec read_one () = 
  try 
    match read_int_opt () with
    | None -> 0
    | Some i -> i + read_one ()
  with 
    End_of_file -> 0;;

let rec get_highest (f : unit -> int) = 
  match f () with 
  | 0 -> (0, 0, 0)
  | x -> max (get_highest f) x ;;

let sum (a, b, c) = a + b + c;;
  
print_int (sum (get_highest read_one));;