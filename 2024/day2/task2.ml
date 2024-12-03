let read_contents file_name =
  In_channel.with_open_bin file_name In_channel.input_all

let split_into_lines contents = String.split_on_char '\n' contents
let split_into_elements line = String.split_on_char ' ' line
let clean_line line = List.filter (fun s -> s <> "") line

type diff = Dec | Inc | ZeroOrLarge

let check_pair x y : diff =
  let diff = x - y in
  match diff with
  | d when d = 0 -> ZeroOrLarge
  | d when d > 3 -> ZeroOrLarge
  | d when d < -3 -> ZeroOrLarge
  | d when d > 0 -> Dec
  | d when d < 0 -> Inc
  | _ -> failwith "All cases should be covered"

let rec check_levels lst idx prev_change =
  match lst with
  | [] -> (None, 1)
  | _ :: [] -> (None, 1)
  | x :: y :: t -> (
      let change = check_pair x y in
      match (change, prev_change) with
      | ZeroOrLarge, _ -> (Some idx, 0)
      (* Handle case where [4; 5; 4; 3; 1 ] *)
      | Dec, Some Inc when idx = 1 -> (Some 0, 0)
      | Inc, Some Dec when idx = 1 -> (Some 0, 0)
      | Dec, Some Inc -> (Some idx, 0)
      | Inc, Some Dec -> (Some idx, 0)
      | _ -> check_levels (y :: t) (idx + 1) (Some change))

let remove_at_idx idx lst =
  let result = List.filteri (fun i _ -> i <> idx) lst in
  result

let check_levels_retry lst =
  let bad_idx, initial_safe = check_levels lst 0 None in
  match (bad_idx, initial_safe) with
  | None, initial_safe -> initial_safe
  | Some idx, 0 ->
      if snd (check_levels (remove_at_idx idx lst) 0 None) = 0 then
        snd (check_levels (remove_at_idx (idx + 1) lst) 0 None)
      else 1
  | Some _, _ -> failwith "should not return bad index with non-zero safe flag"

let solve file_name =
  let contents = read_contents file_name in
  let result =
    split_into_lines contents
    |> List.map (fun x ->
           split_into_elements x |> clean_line |> List.map int_of_string)
    |> List.fold_left (fun acc x -> check_levels_retry x + acc) 0
  in
  result

let solve_print file_name =
  let result = solve file_name in
  print_endline (string_of_int result)
