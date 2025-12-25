# 配置文件

Rumia Loader 可用的配置字段，加载器需要这些字段来启动加载了 Rumia 的《植物大战僵尸》程序。

## 配置文件字段

---

### path

- **类型**: string

《植物大战僵尸》游戏可执行程序的路径。Rumia Loader 将携带 Rumia 启动这个路径指向的程序。

---

### force_launch

- **类型**: bool
- **默认值**: `false`

如果配置为 `true`，则 Rumia Loader 将不会检查 `path` 字段指向的目标是否可用，直接进行加载尝试。仅当你知道在做什么时才应该启用它。

如果 Rumia 启动失败，此项会被自动设置显式为 `false`
