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

(* let pairwise_abs_diff x y = abs (x - y) *)

let find_or_add tbl key =
  let current =
    match Hashtbl.find_opt tbl key with Some v -> v + 1 | None -> 1
  in Hashtbl.replace tbl key current
  

let rec count_in_y lst1 lst2 acc =
  match (lst1, lst2) with
  | [], _ -> (lst1, lst2)
  | _, [] -> (lst1, lst2)
  | x :: xs, y :: ys ->
      (* REPLACE with Map *)
      if x = y then count_in_y xs ys (find_or_add acc x)
      else if x < y then count_in_y xs ys acc
      else count_in_y xs ys acc

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
    (* |> fun (lst1, lst2) -> *)
    (*     List.map2  *)
  in
  result

let solve_print file_name =
  let result = solve file_name in
  print_endline (string_of_int result)
