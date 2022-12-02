let rec process_lines () = 
  try
    let line = read_line () in
    let a = String.get line 0 in
    let b = String.get line 2 in
    process_lines () + match b with
    | 'X' -> 1 + (
      match a with
      | 'A' -> 3
      | 'B' -> 0
      | 'C' -> 6
      | _ -> 0)
    | 'Y' -> 2 + (
      match a with
      | 'A' -> 6
      | 'B' -> 3
      | 'C' -> 0
      | _ -> 0)
    | 'Z' -> 3 + (
      match a with
      | 'A' -> 0
      | 'B' -> 6
      | 'C' -> 3
      | _ -> 0)
    | _ -> 0
  with
  _ -> 0;;
  
print_int (process_lines ());;