_G.control_ws=http.websocket("ws://46.117.242.33:1312");
_G.ckie = {
  ingest_packets = function()
    repeat
        json, bin = control_ws.receive()
        data = textutils.unserializeJSON(json)
        ckie["handle_" .. data.type](data)
    until false
  end,
  send_packet = function(data)
    json = textutils.serializeJSON(data)
    control_ws.send(json, false)
  end,

  handle_Eval = function(packet)
    print("eval")
    send_packet{ type = "EvalResult", result = load(packet.code)()}
  end
};

ckie.ingest_packets()
