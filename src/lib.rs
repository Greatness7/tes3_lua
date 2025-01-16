use mlua::prelude::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[mlua::lua_module]
fn tes3_lua(lua: &Lua) -> LuaResult<LuaTable> {
    tes3::esp::lua_module(lua)
}
