let read_string filepath = 
  let channel = open_in_bin filepath in
    try
      let file = really_input_string channel (in_channel_length channel) in
        close_in channel;
        file
    with e ->
      close_in_noerr channel;
      raise e
    ;;
  ;;
;;

let rec read_lines_internal channel list =
  try
    let line = input_line channel in
      match line with
      | "" -> close_in channel; list
      | _  -> read_lines_internal channel (list  @ [line])
  with _ ->
    close_in_noerr channel;
    list
  ;;
;;

let read_lines filepath =
  let channel = open_in filepath in
    try
      let file = read_lines_internal channel [] in
        file
    with e ->
      close_in_noerr channel;
      raise e
    ;;
  ;;
;;