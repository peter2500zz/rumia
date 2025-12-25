
# 玩点不一样的《植物大战僵尸》

!!! info 准备工作

    你将会需要《植物大战僵尸》 v1.0.0.1051 版本，本仓库不提供副本，请从 [Plants vs. Zombies](https://www.ea.com/games/plants-vs-zombies/plants-vs-zombies) 自行获取。
    
    当然，你也需要 Rumia，在 Rumia 仓库的 [Docs + Windows x86 Debug Build](https://github.com/peter2500zz/rumia/actions/workflows/docs_and_build.yml) 工作流中可以找到自动构建的 Rumia，或者在 [Release](https://github.com/peter2500zz/rumia/releases) 页面找到稳定版本。解压你找到的压缩包，可以找到 `rumia.dll` 和 `loader.exe`。

想要启动 Rumia？那好像有点困难。Rumia 并不能直接被双击打开，而是需要被加载器加载到《植物大战僵尸》游戏中。将 `rumia.dll` 与 `loader.exe` 放进《植物大战僵尸》的游戏文件夹中，双击 `loader.exe`，如果一切正常，你会看到标题为 `Plants vs. Zombies with Rumia` 的游戏被启动，这就代表你成功了！

Rumia 的核心就在于可以加载 Lua 编写的 Mod。如果你已经启动过带 Rumia 的游戏，你可以找到一个 `mods` 文件夹（你也可以提前创建一个）。将其他作者发布的 Mod 放入 `mods` 文件夹中，下次启动时就会自动加载 Mod，就像 Minecraft 做的那样！

