open OUnit2
open Day01

let test_parse_lines ctxt =
  let lines = [ "10   11"; "20   21"; "30   31"; "40   41" ] in
  let want = ([| 10; 20; 30; 40 |], [| 11; 21; 31; 41 |]) in

  let got = Part1.parse_lines lines in

  assert_equal ~ctxt got want

let test_calculate_part1 ctxt =
  let lines = [ "3   4"; "4   3"; "2   5"; "1   3"; "3   9"; "3   3" ] in
  let want = 11 in

  let got = Part1.calculate lines in

  assert_equal ~ctxt ~msg:(Printf.sprintf "got=%d want=%d" got want) got want

let test_calculate_part2 ctxt =
  let lines = [ "3   4"; "4   3"; "2   5"; "1   3"; "3   9"; "3   3" ] in
  let want = 31 in

  let got = Part2.calculate lines in

  assert_equal ~ctxt ~msg:(Printf.sprintf "got=%d want=%d" got want) got want

let suite =
  "day01"
  >::: [
         "test_parse_lines" >:: test_parse_lines;
         "test_calculate_day1" >:: test_calculate_part1;
         "test_calculate_day2" >:: test_calculate_part2;
       ]

let () = run_test_tt_main suite
