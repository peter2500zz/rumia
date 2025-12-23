# 配置文件

Rumia 提供了一些配置选项，用于调整程序的行为。配置文件从 `conf.yml` 文件中读取

## 配置文件字段

---

### show_terminal

- **类型**: bool
- **默认值**: `false`

控制是否为《植物大战僵尸》游戏程序显示终端，如果显示，日志将同时在终端中被打印。

---

### log_path

- **类型**: string
- **默认值**: `pvz.log`

Rumia 日志写入的文件。

---

### log_level

- **类型**: string
- **默认值**: `Info`
- **可选值**: `Off` | `Error` | `Warn` | `Info` | `Debug` | `Trace`

控制日志记录最小记录级别。
