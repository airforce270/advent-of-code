open Core

let re = Re.Pcre.regexp " +"

let read_data (file_name : string) : string list =
  In_channel.read_lines file_name

let parse_line (line : string) =
  match Re.split re line with
  | [ a; b ] -> (Int.of_string a, Int.of_string b)
  | _ -> failwith (sprintf "Invalid input: %s" line)

let parse_lines (lines : string list) : int array * int array =
  lines |> List.map ~f:parse_line |> List.unzip
  |> fun (first_col, second_col) ->
  (Array.of_list first_col, Array.of_list second_col)

let measure (a, b) = Int.abs (a - b)

let calculate (lines : string list) : int =
  let a, b = parse_lines lines in

  Stdlib.Array.sort compare a;
  Stdlib.Array.sort compare b;

  Stdlib.Array.combine a b |> Array.map ~f:measure
  |> Array.fold ~init:0 ~f:( + )
