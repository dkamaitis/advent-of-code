let read_contents file_name =
  In_channel.with_open_bin file_name In_channel.input_all

let split_into_lines contents = String.split_on_char '\n' contents
let split_into_elements line = String.split_on_char ' ' line
let clean_line line = List.filter (fun s -> s <> "") line

(* (["a"; "b"], ["c"; "d"], ["e"; "f"]) -> (["a"; "c"; "e"], ["b"; "d"; "f"]) *)
let transpose acc line =
  match line with
  | [] -> (fst acc, snd acc)
  | [ first; second ] -> (first :: fst acc, second :: snd acc)
  | _ -> failwith "Expected list of length 2"

let add_or_inc key lst =
  match lst with
  | [] -> (key, 1) :: []
  | (key', value) :: t ->
      if key = key' then (key', value + 1) :: t else (key, 1) :: lst

let rec count_in_y lst1 lst2 acc =
  match (lst1, lst2) with
  | [], _ -> acc
  | _, [] -> acc
  | x :: xs, y :: ys ->
      if x = y then count_in_y (x :: xs) ys (add_or_inc x acc)
      else if x < y then count_in_y xs (y :: ys) acc
      else count_in_y (x :: xs) ys acc

let solve file_name =
  let contents = read_contents file_name in
  let result =
    split_into_lines contents
    |> List.fold_left
         (fun acc x ->
           split_into_elements x |> clean_line |> List.map int_of_string
           |> transpose acc)
         ([], [])
    |> fun (lst1, lst2) ->
    (List.sort Int.compare lst1, List.sort Int.compare lst2)
    |> fun (lst1, lst2) ->
    count_in_y lst1 lst2 []
    |> List.fold_left (fun acc (k, v) -> acc + (k * v)) 0
  in
  result

let solve_print file_name =
  let result = solve file_name in
  print_endline (string_of_int result)
