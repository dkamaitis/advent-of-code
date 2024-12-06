let read_contents file_name =
  In_channel.with_open_bin file_name In_channel.input_all

let split_into_lines contents = String.split_on_char '\n' contents
let split_into_elements line = String.split_on_char ' ' line
let clean_line line = List.filter (fun s -> s <> "") line
let expected = [ ('m', 'u', 'l', '(', '*', ',', '*', ')') ]

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
    (current_digits : char list option * char list option) (enabled : bool)
    (text : char list) =
  match text with
  | [] -> (acc, enabled)
  | h :: t -> (
      if enabled then
        match h with
        | 'm' -> parse 'u' acc (None, None) true t
        | 'u' when expected = 'u' -> parse 'l' acc (None, None) true t
        | 'l' when expected = 'l' -> parse '(' acc (None, None) true t
        | '(' when expected = '(' -> parse '*' acc (None, None) true t
        | '0' .. '9' when expected = '*' ->
            parse '*' acc
              (with_digit h (fst current_digits), snd current_digits)
              true t
        | ',' when expected = '*' -> parse '+' acc current_digits true t
        | '0' .. '9' when expected = '+' ->
            parse '+' acc
              (fst current_digits, with_digit h (snd current_digits))
              true t
        | ')' when expected = '+' ->
            parse 'm' (unwrap_digits current_digits :: acc) (None, None) true t
        | 'd' -> parse 'o' acc (None, None) true t
        | 'o' when expected = 'o' -> parse 'n' acc (None, None) true t
        | 'n' when expected = 'n' -> parse '\'' acc (None, None) true t
        | '\'' when expected = '\'' -> parse 't' acc (None, None) true t
        | 't' when expected = 't' -> parse '[' acc (None, None) true t
        | '(' when expected = '[' -> parse ']' acc (None, None) true t
        | ')' when expected = ']' -> parse 'd' acc (None, None) false t
        | _ -> parse 'm' acc (None, None) true t
      else
        match h with
        | 'd' -> parse 'o' acc (None, None) false t
        | 'o' when expected = 'o' -> parse '(' acc (None, None) false t
        | '(' when expected = '(' -> parse ')' acc (None, None) false t
        | ')' when expected = ')' -> parse 'm' acc (None, None) true t
        | _ -> parse 'd' acc (None, None) false t)

let rec parse_lines (acc : (int * int) list) (enabled : bool)
    (lines : string list) =
  match lines with
  | [] -> List.fold_left (fun acc (a, b) -> acc + (a * b)) 0 acc
  | h :: t ->
      let parse_result =
        string_to_char_list h |> parse 'm' acc (None, None) enabled
      in
      parse_lines (fst parse_result) (snd parse_result) t

let solve file_name =
  let contents = read_contents file_name in
  let result =
    split_into_lines contents |> parse_lines [] true
  in
  result

let solve_print file_name =
  let result = solve file_name in
  print_endline (string_of_int result)
