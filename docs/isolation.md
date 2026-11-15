# 数据隔离

这是隔离操作的唯一说明。其他文档只保留禁止项并链接到这里。

Tauri 的 `app_data_dir()` 使用 Windows 已知文件夹。进程环境变量 `APPDATA` 不会换掉它。普通 `pnpm tauri build --debug --no-bundle` 和 `pnpm tauri dev` 使用系统数据目录，会打开正在使用的曲库和设置。

不要修改 `src-tauri/tauri.conf.json` 来写入 `appDirectoriesOverride`。exe 位于 `src-tauri/target` 或 `dist-portable`，以及二进制里出现某段路径，都不能证明这次构建使用了隔离配置。

## 来源记录

构建脚本在成功后把记录写到 `tests/perf/out/isolation-sources/<exe 的 SHA-256>.json`。这个目录已被忽略。记录至少包含：

- exe 的 SHA-256，以及当时位于允许目录中的构建产物路径
- 配置文件路径和该文件的 SHA-256
- 完整构建命令
- `dataDirKind`：`portable-relative` 或 `absolute-override`
- 预期数据目录

允许目录只限仓库里的 `src-tauri/target` 和 `dist-portable`。没有哈希匹配的记录，验收脚本拒绝启动。配置文件在构建后被改过，哈希对不上，同样拒绝。

两种数据目录分开核对：

- `portable-relative`：配置文件必须是 `src-tauri/tauri.portable.conf.json`，其中的覆盖值必须是相对路径 `./app-data`。数据目录是进程工作目录旁的 `app-data`。
- `absolute-override`：覆盖值必须是绝对路径。规范化之后等于记录里的预期目录，并且位于记录里的测试根目录中。测试根不能是盘符根，并且必须位于 `%TEMP%` 下面。

## 便携构建

```powershell
powershell -File scripts/build-portable.ps1
powershell -File scripts/build-portable.ps1 -Debug
```

`pnpm portable` 和 `pnpm portable:debug` 使用同一份配置，但不会写来源记录。需要记录时运行上面的脚本。

## 一次空库

准备一份未入库的 JSON，只包含绝对路径覆盖，例如 `{ "app": { "appDirectoriesOverride": "C:\\Users\\<you>\\AppData\\Local\\Temp\\music-mine-isolated\\data" } }`。测试根使用已存在的专用空目录，例如 `%TEMP%\music-mine-isolated`。

```powershell
powershell -File scripts/build-isolated.ps1 -Config <json 路径> -TestRoot <测试根>
```

脚本拒绝 `src-tauri/tauri.conf.json`，也拒绝已经入库的配置。

## 语料验收进程

`tests/perf/stage-cover-run.ps1` 只接受带 `portable-relative` 记录的 exe。它把 exe 复制到 `%TEMP%\mm-cover-corpus\run-<label>`，工作目录就是这个运行目录。

启动时写下 `process.json`：PID、启动时间、exe 完整路径。结束时先关闭主窗口，再只结束身份仍然匹配的播放器及其后代进程。PID 被复用，或启动时间、exe 路径对不上时，不结束进程，也不按进程名结束。

删除运行目录前，路径必须落在语料根目录里。遇到指向根目录之外的 junction，直接拒绝删除。
