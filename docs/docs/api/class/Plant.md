# Plant

Plant 是游戏内植物类的定义。

## 方法

以下是 Rect2 的所有方法:

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

判断这个植物实例当前是否仍然在内存中有效。

当植物被移除、死亡或对象失效后，该函数将返回 false。

函数签名:

```lua
fun(self): boolean
```

返回值:

- `boolean`: 植物是否有效。

---

### GetHitbox

获取植物的判定框。

函数签名:

```lua
fun(self): Rect2
```

返回值:

- [Rect2](../class/Rect2.md): 植物的判定区域。

---

### Shoot

如果植物有射击能力，会进行播放射击前摇动画然后射击。在前摇动画内再次触发此函数会打断上一次射击。

函数签名:

```lua
fun(self)
```

---

### ShootRaw

如果植物有射击能力，会零帧起手直接射击，没有前摇动画且不更新射击冷却。

函数签名:

```lua
fun(self)
```
