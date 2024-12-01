open Core
open Day01

let read_data (file_name : string) : string list =
  In_channel.read_lines file_name

let () =
  let data = read_data "input.txt" in

  data |> Part1.calculate |> Printf.printf "Part 1: %d\n";

  data |> Part2.calculate |> Printf.printf "Part 2: %d\n"
