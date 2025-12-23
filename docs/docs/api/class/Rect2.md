# Rect2

Rect2 是 Rumia 内置的矩形类。存储了两个用于表示坐标和长宽的 [Vec2](../class/Vec2.md)

## 字段

以下是 Rect2 的所有字段:

---

### x

表示左上角点横坐标的浮点数。

类型:

- number

---

### y

表示左上角点纵坐标的浮点数。

类型:

- number

---

### width

表示矩形宽度的浮点数。

类型:

- number

---

### height

表示矩形高度的浮点数。

类型:

- number

---

### pos

表示矩形左上角点坐标的二维向量。

类型:

- [Vec2](../class/Vec2.md)

---

### size

表示矩形宽高的二维向量。

类型:

- [Vec2](../class/Vec2.md)

---

### New

给定 x, y, width, height，创建一个 Rect2 以便于计算。

函数签名:

```lua
fun(x: number, y: number, w: number, h: number): Rect2
```

参数:

- `x` number: 左上角点的横坐标。
- `y` number: 左上角点的纵坐标。
- `w` number: 矩形宽度。
- `h` number: 矩形高度。

返回值:

- 创建的 Rect2 类。

---

### Zero

创建一个空矩形。

函数签名:

```lua
fun(): Rect2
```

返回值:

- 所有值都为 0 的空 Rect2

---

## 方法

以下是 Rect2 的所有方法:

---

### Contains

判断一个二维向量是否在自身范围内。

函数签名:

```lua
fun(self, pos: Vec2): boolean
```

参数:

- `pos` [Vec2](../class/Vec2.md): 一个二维向量。

返回值:

- 给定二维向量是否在自身范围内的布尔值。

---

### Collides

判断一个矩形是否与自身碰撞/相交。

函数签名:

```lua
fun(self, other: Rect2): boolean
```

参数:

- `other` Rect2: 另一个矩形。

返回值:

- 给定矩形是否与自身碰撞/相交的布尔值。
