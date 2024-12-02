let read_contents file_name =
  In_channel.with_open_bin file_name In_channel.input_all

let split_into_lines contents = String.split_on_char '\n' contents
let split_into_elements line = String.split_on_char ' ' line
let clean_line line = List.filter (fun s -> s <> "") line

type diff = Dec | Inc | ZeroOrLarge

let check_pair x y =
  let diff = x - y in
  match diff with
  | d when d = 0 -> ZeroOrLarge
  | d when d > 3 -> ZeroOrLarge
  | d when d < -3 -> ZeroOrLarge
  | d when d > 0 -> Dec
  | d when d < 0 -> Inc
  | _ -> failwith "All cases should be covered"

let rec check_levels_aux lst acc =
  match lst with
  | [] -> 1
  | _ :: [] -> 1
  | x :: y :: t ->
      let change = check_pair x y in
      if change = ZeroOrLarge || change != acc then 0
      else check_levels_aux (y :: t) change

let check_levels lst =
  match lst with
  | x :: y :: t -> check_levels_aux (y :: t) (check_pair x y)
  | _ -> 0

let rec sum_tr lst acc =
  match lst with [] -> acc | h :: t -> sum_tr t (h + acc)

let solve file_name =
  let contents = read_contents file_name in
  let result =
    split_into_lines contents
    |> List.map (fun x ->
           split_into_elements x |> clean_line |> List.map int_of_string)
    |> List.fold_left (fun acc x -> check_levels x + acc) 0
  in
  result

let solve_print file_name =
  let result = solve file_name in
  print_endline (string_of_int result)
