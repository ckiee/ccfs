_G.control_ws=http.websocket("ws://46.117.242.33:1312");print(control_ws);control_ws.send('{"type":"Bootstrap"}');data,bin=control_ws.receive();print(load(textutils.unserializeJSON(data).code)());
