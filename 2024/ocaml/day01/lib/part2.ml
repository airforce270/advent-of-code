(* A collaboration with Claude Haiku :) *)

let calculate (lines : string list) : int =
  let a, b = Part1.parse_lines lines in

  let b_freq = Hashtbl.create (Array.length b) in

  Array.iter
    (fun num ->
      let current_count = try Hashtbl.find b_freq num with Not_found -> 0 in
      Hashtbl.replace b_freq num (current_count + 1))
    b;

  Array.fold_left
    (fun acc num ->
      let freq = try Hashtbl.find b_freq num with Not_found -> 0 in
      acc + (num * freq))
    0 a
