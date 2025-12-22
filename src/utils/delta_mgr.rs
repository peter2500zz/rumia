use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use tracing::error;

/// 全局静态 Delta 管理器实例
static DELTA_MGR: OnceLock<Mutex<DeltaManager>> = OnceLock::new();

/// 获取全局 Delta 管理器的可变引用
///
/// # Panics
/// 如果 Mutex 被毒化(poisoned),会先记录错误日志再 panic
pub fn get_delta_mgr() -> std::sync::MutexGuard<'static, DeltaManager> {
    let mutex = DELTA_MGR.get_or_init(|| Mutex::new(DeltaManager::with_capacity(32)));

    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            error!("Delta 管理器的 Mutex 已被毒化");
            panic!("Delta 管理器的 Mutex 已被毒化: {:?}", poisoned);
        }
    }
}

/// 高性能 Delta 管理器
///
/// 为每个更新函数维护独立的 delta 时间,支持:
/// - 多次读取 delta 而不更新
/// - 使用字符串字面量作为唯一标识符
/// - 零拷贝的键访问
pub struct DeltaManager {
    deltas: HashMap<&'static str, DeltaEntry>,
}

struct DeltaEntry {
    last: Instant,
    delta: Duration,
}


impl DeltaManager {
    /// 创建新的 Delta 管理器
    pub fn new() -> Self {
        Self {
            deltas: HashMap::new(),
        }
    }

    /// 创建具有预分配容量的 Delta 管理器
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            deltas: HashMap::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn get_delta(&self, key: &'static str) -> Option<f32> {
        self.deltas
            .get(key)
            .map(|e| e.delta.as_secs_f32())
    }

    #[inline]
    pub fn get_delta_duration(&self, key: &'static str) -> Option<Duration> {
        self.deltas.get(key).map(|e| e.delta)
    }


    #[inline]
    pub fn update_delta(&mut self, key: &'static str) -> f32 {
        let now = Instant::now();

        match self.deltas.get_mut(key) {
            Some(entry) => {
                let d = now - entry.last;
                entry.last = now;
                entry.delta = d;
                d.as_secs_f32()
            }
            None => {
                self.deltas.insert(
                    key,
                    DeltaEntry {
                        last: now,
                        delta: Duration::ZERO,
                    },
                );
                0.0
            }
        }
    }


    /// 更新 delta 时间戳并返回 Duration
    #[inline]
    pub fn update_delta_duration(&mut self, key: &'static str) -> Duration {
        let now = Instant::now();

        match self.deltas.get_mut(key) {
            Some(entry) => {
                let d = now - entry.last;
                entry.last = now;
                entry.delta = d;
                d
            }
            None => {
                self.deltas.insert(
                    key,
                    DeltaEntry {
                        last: now,
                        delta: Duration::ZERO,
                    },
                );
                Duration::ZERO
            }
        }
    }

    /// 检查某个键是否存在
    #[inline]
    pub fn contains_key(&self, key: &'static str) -> bool {
        self.deltas.contains_key(key)
    }

    /// 重置某个键的时间戳为当前时间
    #[inline]
    pub fn reset(&mut self, key: &'static str) {
        self.deltas.insert(key, DeltaEntry { last: Instant::now(), delta: Duration::ZERO });
    }

    /// 移除某个键
    #[inline]
    pub fn remove(&mut self, key: &'static str) -> bool {
        self.deltas.remove(key).is_some()
    }

    /// 清空所有 delta
    pub fn clear(&mut self) {
        self.deltas.clear();
    }

    /// 获取管理的 delta 数量
    pub fn len(&self) -> usize {
        self.deltas.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.deltas.is_empty()
    }
}

impl Default for DeltaManager {
    fn default() -> Self {
        Self::new()
    }
}
