# ModCallbacks.AT_GAME_KEY_DOWN

!!! info "回调函数签名"

    ```lua
    fun(keycode: KeyCode): boolean?
    ```

    参数:

    - `keycode` [KeyCode](../alias/KeyCode.md): 按下的键码

    返回值:

    - 如果是 `true` 则会消费此次事件

在游戏中键盘按键被按下时会触发此回调。按下的按键由键码表示。

如果返回 `true`，则优先级低于此 Mod 的其他 Mod 将不会触发此回调，游戏也不会响应此次事件。
