---@meta

---将日志写入到终端中
---@class Log
---@field info fun(...: any): nil @输出信息级别日志
---@field warn fun(...: any): nil @输出警告级别日志
---@field error fun(...: any): nil @输出错误级别日志
---@field debug fun(...: any): nil @输出调试级别日志
---@field trace fun(...: any): nil @输出追踪级别日志
Log = Log

---注册一个模组
---@param name string @模组名称
---@return Mod @返回模组对象
function RegisterMod(name) end

---@alias CallbackFunction fun(...): ...

---模组对象
---@class Mod
---@field name string @模组名称
---@field priority integer @模组优先级
---@field AddCallback fun(self: Mod, callback: integer, function: CallbackFunction): nil @添加回调函数

---回调点
---@class ModCallbacks
---@field AT_NEW_COIN integer @游戏关卡添加掉落物，fun(args: NewCoinArgs): nil
---@field AT_NEW_ZOMBIE integer @游戏关卡生成僵尸，fun(args: NewZombieArgs): nil
---@field AT_ZOMBIE_INIT integer @僵尸初始化，fun(zombie: Zombie): nil
---@field AT_ZOMBIE_UPDATE integer @僵尸更新，fun(zombie: Zombie): nil
ModCallbacks = ModCallbacks

---游戏类定义

---@class Zombie
---@field id integer @僵尸的id

---回调参数定义
---@class NewCoinArgs
---@field x integer @掉落物产生在x坐标
---@field y integer @到落伍产生在y坐标
---@field coin_type integer @掉落物的类型
---@field coin_motion integer @掉落物的运动方式

---@class ArgsNewZombie
---@field row integer @僵尸所在的行
---@field zombie_type integer @僵尸的类型
---@field from_wave integer @僵尸来自的波次

