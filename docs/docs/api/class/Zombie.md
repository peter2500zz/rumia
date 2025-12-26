# Zombie

!!! info

    本文部分内容由 AI 辅助编写，如果发现错误请提交 issue。

Zombie 是游戏内僵尸类的定义。

## 方法

以下是 Zombie 的所有方法:

---

### SetAttr

向这个实例中写入自定义数据，数据可以是任意能被序列化的内容。写入的数据会保存到存档中。

写入的数据可以跨 Mod 访问，如果不希望冲突你可能需要复杂一点的键名。

函数签名:

```lua
fun(self, key: string, value: Serializable)
```

参数:

- `key` string: 用于检索数据的键名。
- `value` [Serializable](../alias/Serializable.md): 可被序列化的任意数据。

---

### GetAttr

从这个实例中读取自定义数据。如果键存在可以跨 Mod 访问数据。

函数签名:

```lua
fun(self, key: string): Serializable?
```

参数:

- `key` string: 用于检索数据的键名。

返回值:

- 如果存在数据，将会返回未知类型但一定可序列化的结果。否则返回 nil。

---

### RemoveAttr

从这个实例中删除自定义数据。可以删除其他 Mod 设定的数据。

函数签名:

```lua
fun(self, key: string)
```

参数:

- `key` string: 被删除数据的键。

---

### IsValid

判断这个僵尸实例当前是否仍然在内存中有效。

当僵尸被移除、死亡或对象失效后，该函数将返回 false。

函数签名:

```lua
fun(self): boolean
```

返回值:

- `boolean`: 僵尸是否有效。

---

### GetPos

获取僵尸当前所在的坐标。

函数签名:

```lua
fun(self): Vec2
```

返回值:

- [Vec2](../class/Vec2.md): 僵尸的坐标。

---

### SetPos

设定僵尸的坐标。

该函数会同时自动修正僵尸所在的行，设定后僵尸会自动走到最接近的行。

函数签名:

```lua
fun(self, pos: Vec2)
```

参数:

- `pos` [Vec2](../class/Vec2.md): 要设定的坐标。

---

### SetPosRaw

直接设定僵尸的世界坐标，不会自动设定所在行。僵尸在被设定坐标后会尝试返回原来的行。

函数签名:

```lua
fun(self, pos: Vec2)
```

参数:

- `pos` [Vec2](../class/Vec2.md): 要设定的坐标。

---

### GetRow

获取僵尸当前所在的行。

函数签名:

```lua
fun(self): integer
```

返回值:

- `integer`: 僵尸所在的行号。

---

### SetRow

设定僵尸所在的行。

函数签名:

```lua
fun(self, row: integer)
```

参数:

- `row` integer: 要设定的行号。

---

### GetHitbox

获取僵尸的命中判定框。

函数签名:

```lua
fun(self): Rect2
```

返回值:

- [Rect2](../class/Rect2.md): 僵尸的命中判定区域。

---

### GetHitboxRelative

获取僵尸的命中判定框，原点为僵尸自身坐标。

函数签名:

```lua
fun(self): Rect2
```

返回值:

- [Rect2](../class/Rect2.md): 相对于僵尸坐标的命中判定区域。

---

### GetAtkbox

获取僵尸的攻击判定框。

函数签名:

```lua
fun(self): Rect2
```

返回值:

- [Rect2](../class/Rect2.md): 僵尸的攻击判定区域。

---

### GetAtkboxRelative

获取僵尸的攻击判定框，原点为僵尸自身坐标。

函数签名:

```lua
fun(self): Rect2
```

返回值:

- [Rect2](../class/Rect2.md): 相对于僵尸坐标的攻击判定区域。

---

### GetBodyHp

获取僵尸本体的当前血量。

函数签名:

```lua
fun(self): integer
```

返回值:

- `integer`: 当前血量。

---

### SetBodyHp

设定僵尸本体的当前血量。

函数签名:

```lua
fun(self, hp: integer)
```

参数:

- `hp` integer: 要设定的血量。

---

### GetBodyHpMax

获取僵尸本体的血量上限。

函数签名:

```lua
fun(self): integer
```

返回值:

- `integer`: 血量上限。

---

### SetBodyHpMax

设定僵尸本体的血量上限。

函数签名:

```lua
fun(self, hp_max: integer)
```

参数:

- `hp_max` integer: 要设定的血量上限。

---

### GetHelmetHp

获取僵尸头部护具的当前血量。

函数签名:

```lua
fun(self): integer
```

返回值:

- `integer`: 当前护具血量。

---

### SetHelmetHp

设定僵尸头部护具的当前血量。

函数签名:

```lua
fun(self, hp: integer)
```

参数:

- `hp` integer: 要设定的护具血量。

---

### GetHelmetHpMax

获取僵尸头部护具的血量上限。

函数签名:

```lua
fun(self): integer
```

返回值:

- `integer`: 护具血量上限。

---

### SetHelmetHpMax

设定僵尸头部护具的血量上限。

函数签名:

```lua
fun(self, hp_max: integer)
```

参数:

- `hp_max` integer: 要设定的护具血量上限。

---

### GetShieldHp

获取僵尸身前护具的当前血量。

函数签名:

```lua
fun(self): integer
```

返回值:

- `integer`: 当前护具血量。

---

### SetShieldHp

设定僵尸身前护具的当前血量。

函数签名:

```lua
fun(self, hp: integer)
```

参数:

- `hp` integer: 要设定的护具血量。

---

### GetShieldHpMax

获取僵尸身前护具的血量上限。

函数签名:

```lua
fun(self): integer
```

返回值:

- `integer`: 护具血量上限。

---

### SetShieldHpMax

设定僵尸身前护具的血量上限。

函数签名:

```lua
fun(self, hp_max: integer)
```

参数:

- `hp_max` integer: 要设定的护具血量上限。
