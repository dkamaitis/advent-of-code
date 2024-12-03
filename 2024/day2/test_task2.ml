open OUnit2
open Day2.Task2


let make_check_levels_test name expected_output input =
  name >:: fun _ ->
  assert_equal expected_output (check_levels_retry input) ~printer:string_of_int

let tests =
  "tests for check_levels"
  >::: [
         make_check_levels_test "safe1" 1 [ 7; 6; 4; 2; 1 ];
         make_check_levels_test "unsafe1" 0 [ 1; 2; 7; 8; 9 ];
         make_check_levels_test "unsafe2" 0 [ 9; 7; 6; 2; 1 ];
         make_check_levels_test "saferm1" 1 [ 1; 3; 2; 4; 5 ];
         make_check_levels_test "saferm2" 1 [ 8; 6; 4; 4; 1 ];
         make_check_levels_test "safe2" 1 [ 1; 3; 6; 7; 9 ];
         make_check_levels_test "saferm3" 1 [ 9; 3; 6; 7; 9 ];
         make_check_levels_test "saferm4" 1 [ 1; 3; 6; 7; 11 ];
         make_check_levels_test "saferm5" 1 [ 1; 3; 6; 9; 9 ];
         make_check_levels_test "saferm6" 1 [ 7; 7; 4; 2; 1 ];
         make_check_levels_test "saferm7" 1 [ 7; 6; 4; 2; 2 ];
         make_check_levels_test "saferm8" 1 [ 7; 6; 4; 2; 3 ];
         make_check_levels_test "saferm9" 1 [ 4; 5; 4; 3; 1 ];
       ]

let _ = run_test_tt_main tests

let file_name = "input"
let _ = solve_print file_name
