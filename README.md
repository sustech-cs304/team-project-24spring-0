# Moras

## 开发规范

- 在各个分支开发，完成后在 github 提交 pull request，审核后合并到 main 分支
- 分支命名规则:
  - feature-xxx（新功能）
  - fix-xxx（修复 bug）
  - refactor-xxx（重构）
  - doc-xxx（文档）
- commit message 规范：使用[约定式提交](https://www.conventionalcommits.org/zh-hans/v1.0.0/)
- rust 代码提交时需要进行格式化，否则无法通过 CI，可以使用 "src-tauri/commit.sh" 一键格式化并提交 commit (windows 用户使用 commit.bat)

