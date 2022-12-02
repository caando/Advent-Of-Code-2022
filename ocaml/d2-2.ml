let rec process_lines () = 
  try
    let line = read_line () in
    let a = String.get line 0 in
    let b = String.get line 2 in
    process_lines () + match b with
    | 'X' -> 0 + (
      match a with
      | 'A' -> 3
      | 'B' -> 1
      | 'C' -> 2
      | _ -> 0)
    | 'Y' -> 3 + (
      match a with
      | 'A' -> 1
      | 'B' -> 2
      | 'C' -> 3
      | _ -> 0)
    | 'Z' -> 6 + (
      match a with
      | 'A' -> 2
      | 'B' -> 3
      | 'C' -> 1
      | _ -> 0)
    | _ -> 0
  with
  _ -> 0;;
  
print_int (process_lines ());;