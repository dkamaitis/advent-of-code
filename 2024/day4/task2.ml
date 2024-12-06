let read_contents file_name =
  In_channel.with_open_bin file_name In_channel.input_all

let split_into_lines contents =
  String.split_on_char '\n' contents |> Array.of_list

let split_into_chars (lines : string array) : char array array =
  Array.map (fun s -> s |> String.to_seq |> Array.of_seq) lines

let get_opt arr i =
  if i >= 0 && i < Array.length arr then Some arr.(i) else None

let rec extract_word (chars : char array array) x y n acc dx dy : char list =
  let line = get_opt chars y in
  let current_char =
    match line with None -> None | Some line -> get_opt line x
  in
  let new_x = x + dx in
  let new_y = y + dy in
  match (n, current_char) with
  | 0, _ -> acc
  | _, None -> acc
  | _, Some character ->
      extract_word chars new_x new_y (n - 1) (character :: acc) dx dy

let char_list_to_strs lst =
  (lst, List.rev lst) |> fun (a, b) ->
  (List.to_seq a |> String.of_seq, List.to_seq b |> String.of_seq)

let parse (chars : char array array) (target_word : string) n x y =
  let words =
    ( extract_word chars x y n [] 1 1 |> char_list_to_strs,
      extract_word chars x (y + 2) n [] 1 (-1) |> char_list_to_strs )
  in
  let (down_fwd, down_rev), (up_fwd, up_rev) = words in
  if
    (up_fwd = target_word || up_rev = target_word)
    && (down_fwd = target_word || down_rev = target_word)
  then 1
  else 0

let solve file_name =
  let contents = read_contents file_name in
  let char_grid = split_into_lines contents |> split_into_chars in
  let height, width = (Array.length char_grid, Array.length char_grid.(0)) in
  let result =
    Array.init height (fun y ->
        Array.init width (fun x -> parse char_grid "MAS" 3 x y))
    |> Array.fold_left (fun acc line -> acc + Array.fold_left ( + ) 0 line) 0
  in
  result

let solve_print file_name =
  let result = solve file_name in
  print_endline (string_of_int result)
