```lua
local lib = require("tes3_lua")

local plugin = lib.load_plugin("Path/To/Morrowind.esm")

-- You can use `ipairs` instead if your version of lua supports the `__ipairs` metamethod.
for i = 1, #plugin.objects do
    local object = plugin.objects[i]

    if object.id == "fargoth" then
        print(object)
        break
    end

end
```
