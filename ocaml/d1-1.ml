let max a b =
  if a > b then a
  else b;;

let rec read_one () = 
  try 
    match read_int_opt () with
    | None -> 0
    | Some i -> i + read_one ()
  with 
    End_of_file -> 0;;

let rec get_highest (f : unit -> int) = 
  match f () with 
  | 0 -> 0
  | x -> (max x (get_highest f)) ;;
  
print_int (get_highest read_one);;