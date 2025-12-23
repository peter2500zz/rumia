# Mod

Rumia 提供用于表示这个 Mod 的类。如果在一个文件中定义了多个 Mod 类，它们将被算作不同的 Mod。有关多 Mod 之间的行为，请参阅[Mod与环境隔离](../sandbox.md)

## 字段

以下是 Mod 的所有字段:

---

### name

模组的显示名称，以 UTF-8 存储。

类型:

- string

---

### priority

模组的加载优先级，暂未实现。

类型:

- integer

---

## 方法

以下是 Mod 的所有方法:

---

### AddCallback

为此 Mod 注册一个回调函数，当游戏触发这个事件时，函数将被执行。

函数签名:

```lua
fun(self, callback: ModCallback, function: CallbackFunction)
```

参数:

- `callback` [ModCallback](../alias/ModCallback.md): 要注册的回调点
- `function` [CallbackFunction](../alias/CallbackFunction.md): 回调函数
