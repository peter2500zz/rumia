use mlua::prelude::*;
use std::ffi::c_void;

use crate::{
    pvz::{
        board::board::get_board,
        plant::{Fire, FireWithoutTarget},
    },
    save::PROFILE_MANAGER,
    utils::{Rect2, Vec2, data_array::HasId},
};

#[repr(C)]
#[derive(Debug)]
pub struct Plant {
    /// 0x0 基址
    pub base_address: *mut c_void,
    /// 0x4 当前游戏信息和对象
    pub current_game_info: *mut c_void,
    /// 0x8 植物碰撞箱
    pub hitbox: Rect2<i32>,
    /// 0x18 为true时可见
    pub is_visible: bool,
    _padding_19: [u8; 3],
    /// 0x1C 所在行数
    pub row: i32,
    /// 0x20 图像图层
    pub image_layer: i32,
    /// 0x24 植物类型(模仿植物此处为被模仿的植物类型)
    pub plant_type: i32,
    /// 0x28 所在列数
    pub col: i32,
    /// 0x2C 无动画抖动时间
    pub frame_time: i32,
    /// 0x30 无动画抖动帧索引
    pub frame_index: i32,
    /// 0x34 无动画抖动帧时间
    pub frame_duration: i32,
    /// 0x38 无动画抖动帧计数
    pub frame_counter: i32,
    /// 0x3C 植物状态
    pub plant_state: i32,
    /// 0x40 当前血量
    pub hp: i32,
    /// 0x44 血值上限
    pub hp_max: i32,
    /// 0x48 植物子类型
    pub plant_subtype: i32,
    /// 0x4C 植物消失倒计时
    pub disappear_countdown: i32,
    /// 0x50 灰烬冰核三叶草生效倒计时
    pub effect_countdown: i32,
    /// 0x54 属性倒计时
    pub attribute_countdown: i32,
    /// 0x58 触发发射/生产物品倒计时
    pub shoot_countdown: i32,
    /// 0x5C 触发发射/生产物品时间间隔
    pub shoot_interval: i32,
    /// 0x60~0x6C [废弃]植物矩形
    _deprecated_rect: [i32; 4],
    /// 0x70~0x7C [废弃]植物攻击矩形
    _deprecated_attack_rect: [i32; 4],
    /// 0x80 炮准心横坐标-47
    pub cannon_target_x: i32,
    /// 0x84 炮准心纵坐标
    pub cannon_target_y: i32,
    /// 0x88 所在行
    pub target_row: i32,
    /// 0x8C 粒子系统ID(大喷菇和忧郁菇喷雾,花园植物发光)
    pub particle_system_id: i32,
    /// 0x90 子弹生成倒计时
    pub projectile_countdown: i32,
    /// 0x94 植物本体动画ID
    pub body_anim_id: u32,
    /// 0x98 豌豆头的动画ID/三线射手上方头的动画ID
    pub head_anim_id1: u32,
    /// 0x9C 三线射手中间头的动画ID
    pub head_anim_id2: u32,
    /// 0xA0 三线射手下方头的动画ID
    pub head_anim_id3: u32,
    /// 0xA4 眨眼动画ID
    pub blink_anim_id: u32,
    /// 0xA8 土豆雷闪灯动画ID
    pub light_anim_id: u32,
    /// 0xAC 蘑菇睡觉时zzz动画ID
    pub sleep_anim_id: u32,
    /// 0xB0 眨眼倒计时
    pub blink_countdown: i32,
    /// 0xB4 被啃50cs倒计时
    pub eaten_countdown: i32,
    /// 0xB8 发光倒计时
    pub glow_countdown: i32,
    /// 0xBC 闪光倒计时
    pub flash_countdown: i32,
    /// 0xC0 图像偏移坐标
    pub image_offset: Vec2<f32>,
    /// 0xC8 吸收物品坐标
    pub attract_pos: Vec2<f32>,
    /// 0xD0 吸收物品目标位置偏移
    pub attract_target_offset: Vec2<f32>,
    /// 0xD8 磁力菇吸取物品类型,吸金磁吸收的第一个物品
    pub attract_item_type: i32,
    /// 0xDC~0x128 吸金磁吸收的剩余4个物品 (0x4C字节 = 19个i32)
    pub attract_items: [i32; 19],
    /// 0x128 这里应该直接到 0x12C，中间有4字节gap
    _padding_128: i32,
    /// 0x12C 攻击目标僵尸ID
    pub target_zombie_id: i32,
    /// 0x130 蘑菇倒计时
    pub mushroom_countdown: i32,
    /// 0x134 蹦极抓住的状态(0没被抓住,1被抓住,2抱走)
    pub bungee_grab_state: i32,
    /// 0x138 是否为模仿植物/模仿者模仿的植物类型(非模仿植物-1,模仿者变身时为对应植物,变身后48)
    pub imitater_type: i32,
    /// 0x13C 方向-1为右,1为左
    pub direction: i32,
    /// 0x140 0和1之间变换
    pub toggle: u8,
    /// 0x141 true则植物消失
    pub is_dead: bool,
    /// 0x142 true则植物压扁
    pub is_squashed: bool,
    /// 0x143 true则植物睡着
    pub is_asleep: bool,
    /// 0x144 true则植物在 Board 上
    pub on_board: bool,
    /// 0x145 true则植物发亮
    pub is_glowing: bool,
    _padding_146: [u8; 2],
    /// 0x148 植物ID(结构为[序列号,编号],序列号与编号各占2字节)
    id: i32,
}
const _: () = assert!(std::mem::size_of::<Plant>() == 0x14C);

impl HasId for Plant {
    const NAMESPACE: &'static str = "Plant";

    fn id(&self) -> i32 {
        self.id
    }
}

/// 尝试通过索引从 Board 中的 plants 对象池中获取植物指针
///
/// 如果无法访问植物会返回 None
pub fn get_plant(id: i32) -> LuaResult<*mut Plant> {
    get_board().and_then(|board| unsafe {
        if let Some(plant) = (*board).plants.get_ptr(id) {
            Ok(plant)
        } else {
            Err(LuaError::MemoryError(format!("Plant({}) 不可访问", id)))
        }
    })
}

/// 尝试通过索引从 Board 中的 plants 对象池中获取植物并执行操作
///
/// 如果无法访问植物会返回错误
pub fn with_plant<T>(id: i32, f: impl FnOnce(&mut Plant) -> LuaResult<T>) -> LuaResult<T> {
    get_plant(id).and_then(|plant| unsafe { f(&mut *plant) })
}

impl LuaUserData for Plant {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // 外部数据
        methods.add_method("SetAttr", |_, this, (key, value)| {
            PROFILE_MANAGER.lock().unwrap().set_attr(this, key, value)
        });
        methods.add_method("GetAttr", |lua, this, key| {
            Ok(PROFILE_MANAGER.lock().unwrap().get_attr(lua, this, key))
        });
        methods.add_method("RemoveAttr", |_, this, key| {
            Ok(PROFILE_MANAGER.lock().unwrap().remove_attr(this, key))
        });

        // 如果植物被从内存里清理掉了，就给 false
        methods.add_method("IsValid", |_, this, ()| Ok(get_plant(this.id()).is_ok()));

        methods.add_method("GetHitbox", |_, this, ()| {
            with_plant(this.id(), |plant| Ok(plant.hitbox))
        });

        methods.add_method("Shoot", |_, this, ()| {
            with_plant(this.id(), |plant| {
                FireWithoutTarget(plant, this.row, 0);

                Ok(())
            })
        });

        methods.add_method("ShootRaw", |_, this, ()| {
            with_plant(this.id(), |plant| {
                Fire(plant, 0 as _, this.row, 0);

                Ok(())
            })
        });
    }
}
