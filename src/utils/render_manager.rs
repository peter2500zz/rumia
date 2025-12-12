use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use tracing::info;

use crate::mods::LuaRegistration;
use crate::pvz::graphics::graphics::Graphics;

// ==========================================
// 1. 基础定义 (Layer 和 Graphics)
// ==========================================

#[allow(non_snake_case, non_upper_case_globals)]
pub mod RenderLayer {
    pub const Debug: i64 = 0;
    pub const UI: i64 = 1;
    pub const Board: i64 = 2;
}

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();

        let render_layers = lua.create_table()?;

        render_layers.set("Debug", RenderLayer::Debug)?;
        render_layers.set("UI", RenderLayer::UI)?;
        render_layers.set("Board", RenderLayer::Board)?;

        globals.set("RenderLayers", render_layers)?;

        Ok(())
    })
}

// ==========================================
// 2. 渲染管理器核心结构
// ==========================================

/// 渲染任务类型别名
/// 使用 Box<dyn FnOnce> 因为每个闭包捕获的环境不同
/// 使用 Send 标记允许闭包在不同线程创建并发送给管理器
type RenderTask = Box<dyn FnOnce(*mut Graphics) + Send>;

pub struct RenderManager {
    // 核心数据结构：Key是层级，Value是该层级的任务队列
    // 优化点：Vec 在 clear() 后会保留 capacity，避免每帧重新分配内存
    queues: HashMap<i64, Vec<RenderTask>>,
}

impl RenderManager {
    fn new() -> Self {
        Self {
            queues: HashMap::new(),
        }
    }

    /// 提交渲染任务
    /// - layer: 目标渲染层
    /// - task: 闭包，接受 *mut Graphics
    pub fn submit<F>(&mut self, layer: i64, task: F)
    where
        F: FnOnce(*mut Graphics) + Send + 'static,
    {
        // 获取或创建该层对应的队列，并推入任务
        self.queues
            .entry(layer)
            .or_insert_with(|| Vec::with_capacity(100)) // 预分配容量优化
            .push(Box::new(task));
    }

    /// 处理指定层的渲染
    /// - layer: 当前要处理的层
    /// - gfx: 实际的图形上下文指针
    ///
    /// 性能关键点：这里使用 drain 或者迭代后 clear，而不是 remove entry。
    /// 这样 HashMap 的结构和 Vec 的堆内存空间得以保留到下一帧复用。
    pub fn process_layer(&mut self, layer: i64, gfx: *mut Graphics) {
        if let Some(queue) = self.queues.get_mut(&layer) {
            if queue.is_empty() {
                return;
            }

            // 逐个执行闭包
            for task in queue.drain(..) {
                // 安全性注意：这里调用者必须保证 gfx 指针有效
                task(gfx);
            }

            // drain 之后 queue 变空，但 capacity 保持不变
        }
    }

    /// 结束帧，清理残留（如果需要）
    /// 大多数情况下 process_layer 已经清空了队列。
    /// 这个函数主要用于防止逻辑错误导致某些层未被渲染而堆积内存。
    pub fn finish_frame(&mut self) {
        for queue in self.queues.values_mut() {
            // 确保清空，但保留 capacity
            queue.clear();
        }
    }
}

// ==========================================
// 3. 全局单例接口
// ==========================================

/// 全局单例存储
static MANAGER: OnceLock<Mutex<RenderManager>> = OnceLock::new();

/// 获取管理器单例的内部辅助函数
fn get_manager() -> &'static Mutex<RenderManager> {
    MANAGER.get_or_init(|| {
        info!("初始化渲染管理器");
        Mutex::new(RenderManager::new())
    })
}

// --- 公共 API (对外暴露的接口) ---

/// 对外暴露的提交函数
pub fn submit_render_task<F>(layer: i64, f: F)
where
    F: FnOnce(*mut Graphics) + Send + 'static,
{
    // lock().unwrap() 在这里是可以接受的，因为如果 Poisoned 了说明程序已经崩了
    get_manager().lock().unwrap().submit(layer, f);
}

/// 对外暴露的层渲染执行函数
pub fn execute_layer_render(layer: i64, gfx: *mut Graphics) {
    get_manager().lock().unwrap().process_layer(layer, gfx);
}

/// 对外暴露的帧结束函数
pub fn finish_render_frame() {
    get_manager().lock().unwrap().finish_frame();
}

// // ==========================================
// // 4. 使用示例 (Test / Mock)
// // ==========================================

// fn main() {
//     // 模拟实际的 Graphics 对象
//     let mut graphics = Graphics { id: 1 };
//     let gfx_ptr: *mut Graphics = &mut graphics;

//     println!("--- Frame Start ---");

//     // 1. 各个系统提交渲染请求 (可以是乱序，甚至多线程)

//     // 玩家逻辑提交到 Characters 层
//     let player_hp = 100;
//     submit_render_task(RenderLayer::Characters, move |gfx| {
//         // move 关键字捕获了 player_hp
//         unsafe {
//             (*gfx).draw(&format!("Player Sprite (HP: {})", player_hp));
//         }
//     });

//     // UI 逻辑提交
//     submit_render_task(RenderLayer::UI, |gfx| {
//         unsafe { (*gfx).draw("Main Menu Button"); }
//     });

//     // 背景逻辑提交
//     submit_render_task(RenderLayer::Background, |gfx| {
//         unsafe { (*gfx).draw("Skybox"); }
//     });

//     // 另一个 Character
//     submit_render_task(RenderLayer::Characters, |gfx| {
//         unsafe { (*gfx).draw("Enemy Sprite"); }
//     });

//     println!("--- Submission Done, Start Rendering ---");

//     // 2. 渲染循环 (通常在主线程)
//     // 根据具体引擎逻辑，按特定顺序请求 Layer 执行

//     unsafe {
//         // 先画背景
//         println!("> Rendering Background Layer:");
//         execute_layer_render(RenderLayer::Background, gfx_ptr);

//         // 再画角色 (注意：之前提交了两个角色任务，这里会一起执行)
//         println!("> Rendering Characters Layer:");
//         execute_layer_render(RenderLayer::Characters, gfx_ptr);

//         // 再画 UI
//         println!("> Rendering UI Layer:");
//         execute_layer_render(RenderLayer::UI, gfx_ptr);
//     }

//     // 3. 帧结束清理
//     finish_render_frame();

//     println!("--- Frame End ---");
// }
