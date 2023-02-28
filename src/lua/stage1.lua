if fs.exists("/ccfs") then
  fs.delete("/ccfs")
end
fs.makeDir("/ccfs")

control_ws.send('{"type":"GetInternalFile","path":"startup.lua"}')
data,bin=control_ws.receive()
f=fs.open("/startup.lua","w")
f.write(textutils.unserializeJSON(data).data)
f.close()
settings.set("shell.allow_startup",true)
settings.save()
os.reboot()
