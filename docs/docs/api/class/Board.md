# Board

Board 代表了游戏关卡，其中包括了许多关卡的特有属性，以及许多与关卡内元素交互的方法。

!!! info

    不建议保存 Board。除非必要，否则请尽可能使用 [LawnApp:GetBoard()](../class/LawnApp.md#getboard) 来获取 Board

## 方法

以下是 LawnApp 的所有方法:

---

### GetUpdateDelta

获取距离与上一逻辑帧过去的时间。

函数签名:

```lua
fun(self): number
```

返回值:

- 距离上一逻辑帧过去的时间（秒）

---

### MousePressing

获取鼠标是否按住（暂停时不进行记录）。

函数签名:

```lua
fun(self): boolean
```

返回值:

- 鼠标是否按住

---

### SetSun

将阳光修改为新的值。

函数签名:

```lua
fun(self, value: integer)
```

参数:

- `value` integer: 新的阳光值

---

### GetZombies

获取场上所有的僵尸。

函数签名:

```lua
fun(self): table<integer, Zombie>
```

返回值:

- 一个表，键是僵尸的 id，值是 [Zombie](../class/Zombie.md) 类

---

### GetZombieById

通过僵尸 id 查询获取特定僵尸，如果没有此 id 的僵尸则返回 nil。

函数签名:

```lua
fun(self, id: integer): Zombie?
```

参数:

- `id` integer: 僵尸的 id

返回值:

- 如果此僵尸存在，返回 [Zombie](../class/Zombie.md) 类，否则返回 nil

---

### AddZombie

在特定行添加一只僵尸。

函数签名:

```lua
fun(self, zombie_type: integer, row: integer, from_wave: integer): Zombie
```

参数:

- `zombie_type` integer: 僵尸的类型 id
- `row` integer: 僵尸所在的行
- `from_wave` integer: 僵尸来自的波次

返回值:

- 返回创建僵尸的 [Zombie](../class/Zombie.md) 类

---

### AddCoin

添加一个掉落物

函数签名:

```lua
fun(self, pos: Vec2, coin_type: CoinType, coin_motion: CoinMotion): Coin
```

参数:

- `pos` [Vec2](../class/Vec2.md): 掉落物的坐标
- `coin_type` [CoinType](../alias/CoinType.md): 掉落物类型
- `coin_motion` [CoinMotion](../alias/CoinMotion.md): 掉落物运动方式

返回值:

- 返回生成掉落物的 [Coin](../class/Coin.md) 类

---

### PosToGridKeepOnBoard

将一个坐标转换为关卡内网格的坐标。且对于网格范围外的坐标，会转换为必定在网格内的网格坐标。

函数签名:

```lua
fun(self, pos: Vec2): Vec2
```

参数:

- `pos` [Vec2](../class/Vec2.md): 原始坐标

返回值:

- 游戏关卡内网格坐标的 [Vec2](../class/Vec2.md)

