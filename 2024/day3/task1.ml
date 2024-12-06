let read_contents file_name =
  In_channel.with_open_bin file_name In_channel.input_all

let split_into_lines contents = String.split_on_char '\n' contents

let string_to_char_list (text : string) : char list =
  List.init (String.length text) (String.get text)

let with_digit (x : char) (digits : char list option) : char list option =
  (* Ideally should use List.rev_map instead *)
  match digits with Some digits -> Some (digits @ [ x ]) | None -> Some [ x ]

let chars_to_int (x : char list) : int =
  x |> List.map (String.make 1) |> String.concat "" |> int_of_string

let unwrap_digits (current_digits : char list option * char list option) :
    int * int =
  match current_digits with
  | None, None -> failwith "should never attempt to unwrap with None"
  | None, _ -> failwith "should never attempt to unwrap with None"
  | _, None -> failwith "should never attempt to unwrap with None"
  | Some x, Some y -> (chars_to_int x, chars_to_int y)

let rec parse (expected : char) (acc : (int * int) list)
    (current_digits : char list option * char list option) (text : char list) =
  match text with
  | [] -> acc
  | h :: t -> (
      match h with
      | 'm' when expected = 'm' -> parse 'u' acc (None, None) t
      | 'u' when expected = 'u' -> parse 'l' acc (None, None) t
      | 'l' when expected = 'l' -> parse '(' acc (None, None) t
      | '(' when expected = '(' -> parse '*' acc (None, None) t
      | '0' .. '9' when expected = '*' ->
          parse '*' acc
            (with_digit h (fst current_digits), snd current_digits)
            t
      | ',' when expected = '*' -> parse '+' acc current_digits t
      | '0' .. '9' when expected = '+' ->
          parse '+' acc
            (fst current_digits, with_digit h (snd current_digits))
            t
      | ')' when expected = '+' ->
          parse 'm' (unwrap_digits current_digits :: acc) (None, None) t
      | _ -> parse 'm' acc (None, None) t)

let solve file_name =
  let contents = read_contents file_name in
  let result =
    split_into_lines contents
    |> List.map (fun x ->
           string_to_char_list x
           |> parse 'm' [] (None, None)
           |> List.fold_left (fun acc (a, b) -> acc + (a * b)) 0)
    |> List.fold_left ( + ) 0
  in
  result

let solve_print file_name =
  let result = solve file_name in
  print_endline (string_of_int result)
