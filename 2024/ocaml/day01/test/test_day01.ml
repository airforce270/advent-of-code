open OUnit2
open Day01.Part1

let test_parse_lines _ = 
  let lines = [
    "10   11";
    "20   21";
    "30   31";
    "40   41";
  ] in

  let want = ([|10;20;30;40|], [|11;21;31;41|]) in

  assert_equal (parse_lines lines) want

let test_calculate _ =
  let lines = [
    "3   4";
    "4   3";
    "2   5";
    "1   3";
    "3   9";
    "3   3";
  ] in

  let want = 11 in

  assert_equal (calculate lines) want

let suite = "day01" >::: [
  "test_parse_lines" >:: test_parse_lines;
  "test_calculate" >:: test_calculate;
]

let () = run_test_tt_main suite

