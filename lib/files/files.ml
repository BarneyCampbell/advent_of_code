let read_lines filepath = 
  let channel = open_in_bin filepath in
    try
      let file = really_input_string channel (in_channel_length channel) in
        close_in channel;
        file
    with e ->
      close_in_noerr channel;
      raise e