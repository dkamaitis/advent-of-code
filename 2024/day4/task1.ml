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

let parse (chars : char array array) (target_word : string) n x y =
  let diffs = [ (1, 0); (0, -1); (1, 1); (1, -1) ] in
  let words =
    List.map
      (fun (dx, dy) ->
        extract_word chars x y n [] dx dy |> fun a ->
        (a, List.rev a) |> fun (a, b) ->
        (List.to_seq a |> String.of_seq, List.to_seq b |> String.of_seq))
      diffs
  in
  List.fold_left
    (fun local_acc x ->
      let fwd, rev = (fst x = target_word, snd x = target_word) in
      if fwd && rev then local_acc + 2
      else if fwd || rev then local_acc + 1
      else local_acc)
    0 words

let solve file_name =
  let contents = read_contents file_name in
  let char_grid = split_into_lines contents |> split_into_chars in
  let height, width = (Array.length char_grid, Array.length char_grid.(0)) in
  let result =
    Array.init height (fun y ->
        Array.init width (fun x -> parse char_grid "XMAS" 4 x y))
    |> Array.fold_left (fun acc line -> acc + Array.fold_left ( + ) 0 line) 0
  in
  result

let solve_print file_name =
  let result = solve file_name in
  print_endline (string_of_int result)
