open Day01.Part1

let () = 
    read_data "input.txt"
    |> calculate 
    |> Printf.printf "Total: %d\n"

