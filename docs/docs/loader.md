# Rumia Loader

Rumia Loader 提供了一种为《植物大战僵尸》加载 Rumia 的方案。它是一个对 Detours DLL 加载功能的简易包装，同时额外做了一些有关游戏本身版本确认的检查（因为 Rumia 只能运行在《植物大战僵尸》 v1.0.0.1051 版本）。

在 Rumia 仓库的 [Docs + Windows x86 Debug Build](https://github.com/peter2500zz/rumia/actions/workflows/docs_and_build.yml) 工作流中可以找到每个提交的自动构建结果 `rumia-auto-debug-build.zip`，其中同时包括 Rumia Loader 和 Rumia，以及它们的 `pdb` 调试文件。如果希望尝试最新的特性，这是个不错的选择。

对于想要使用稳定版的人，很遗憾目前 Rumia 并没有任何的发行版，在将来功能完善后会上传在 Rumia 仓库的 [Release](https://github.com/peter2500zz/rumia/releases) 页面。
