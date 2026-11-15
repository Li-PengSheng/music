# 任务：<标题>

## 基线

- 分支与 HEAD：
- 未提交改动：
- 工作区指纹：验收前运行 `powershell -File scripts/worktree-fingerprint.ps1`。没有保存过的历史任务写「未记录，无法重建」，不要补算。

## 状态

- 状态：进行中 / 受阻 / 待审阅 / 完成
- 依赖与前置任务：
- 下一步：
- 提交：待提交 / <提交哈希>
- 遗留事项的任务 ID：

## 范围

- 要改：
- 不要改：
- 是否涉及音频、数据库、IPC 或用户数据目录：

## 验收

- [ ] 自动测试：PASS / FAIL / NOT RUN
- [ ] 浏览器验证：PASS / FAIL / NOT RUN
- [ ] Windows Tauri 实机：PASS / FAIL / NOT RUN
- [ ] 未打开用户正在使用的曲库和设置

实机必须使用构建出的 exe。会写数据时按 `docs/isolation.md`。不要修改 `src-tauri/tauri.conf.json`。

## 来源

新任务填写。算法见 `docs/testing.md`。

- 任务 ID：
- 时间：
- HEAD：
- 工作区指纹：
- 命令、工具版本、构建配置：
- 产物哈希：
- 三层结果和未验证项：

## 结束报告

- 改动文件：
- 三条验收结果：
- 未验证项：
- Git：只有当前任务明确要求时才提交
