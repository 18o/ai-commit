# AI Commit Tool

[English](README.md) | [中文](README_zh.md)

一个智能的 Git 提交信息生成器，使用 AI 分析你的代码变更并自动创建有意义的、符合规范的提交信息。

## 概述

AI Commit Tool 集成到你的 Git 工作流程中，自动生成遵循 Conventional Commits 规范的高质量提交信息。它分析你的暂存或未暂存的变更，并使用 AI 来制作合适的提交信息，节省时间并确保项目的一致性。

## 功能特性

- **AI 生成提交信息**：自动分析 git diff 并生成遵循常规提交格式的上下文提交信息
- **智能格式选择**：根据变更复杂度自动选择简洁的单行信息或详细的项目符号格式
- **试运行模式**：为未暂存的变更预览生成的信息而不提交
- **修订支持**：为修订之前的提交生成新信息
- **锁文件过滤**：自动忽略常见的锁文件（Cargo.lock、package-lock.json、yarn.lock 等）的分析
- **GPG 签名支持**：与 GPG 签名的提交无缝协作
- **完全可配置**：可自定义 API 设置、忽略模式、行为选项和 AI 提示

## 安装

### 前置要求

- Rust（最新稳定版本）
- Git 仓库
- AI API 密钥（设置为环境变量 `AI_COMMIT_ARK_API_KEY` 或 `ARK_API_KEY`）

### 从源码构建

```bash
git clone <repository-url>
cd ai-commit
cargo build --release
```

### 设置

1. **设置 API 密钥**：

   ```bash
   export AI_COMMIT_ARK_API_KEY="your-api-key-here"
   ```

2. **初始化配置**：
   ```bash
   ai-commit config init
   ```

## 使用方法

### 基本命令

为暂存的变更生成提交信息：

```bash
ai-commit
# 或者明确指定
ai-commit commit
```

为未暂存的变更预览提交信息（试运行模式）：

```bash
# 如果没有找到暂存的变更，将自动进入试运行模式
ai-commit
```

使用新变更修订最后一次提交：

```bash
ai-commit amend
```

### 命令选项

```bash
# 显示生成的信息但不提交
ai-commit --dry-run

# 限制发送给 AI 的上下文（默认：200000 字符）
ai-commit --context-limit 100000

# 修订时使用试运行
ai-commit amend --dry-run
```

### 配置命令

```bash
# 初始化默认配置
ai-commit config init

# 查看当前配置
ai-commit config show

# 获取编辑提示的帮助
ai-commit config edit-prompts
```

### Git 钩子集成

安装 git 钩子以获得自动提交信息协助：

```bash
ai-commit install
```

移除 git 钩子：

```bash
ai-commit uninstall
```

## 配置

工具将配置存储在 `~/.config/ai-commit/config.toml` 中。使用默认设置初始化：

```bash
ai-commit config init
```

### 默认配置结构

```toml
[api]
endpoint = "https://ark.cn-beijing.volces.com/api/v3/chat/completions"
model = "doubao-1-5-pro-32k-250115"
max_tokens = 1000
temperature = 0.7
context_limit = 200000

[commit]
auto_confirm = false
dry_run_by_default = false
ignore_lock_files = true
custom_ignore_patterns = []

[hooks]
enabled = false
hook_types = ["prepare-commit-msg"]

[prompts]
system_prompt = """你是一个专业的软件开发者..."""
user_prompt_template = """分析以下 git diff..."""
simple_prompt_template = """生成简洁的单行提交信息..."""
```

### 配置选项

#### API 设置 (`[api]`)

- `endpoint`：AI 服务端点 URL
- `model`：用于生成的 AI 模型
- `max_tokens`：AI 响应的最大令牌数（默认：1000）
- `temperature`：创造性水平 0.0-1.0（默认：0.7）
- `context_limit`：发送给 AI 的最大字符数（默认：200000）

#### 提交设置 (`[commit]`)

- `auto_confirm`：跳过确认提示（默认：false）
- `dry_run_by_default`：始终在试运行模式下运行（默认：false）
- `ignore_lock_files`：从分析中过滤出锁文件（默认：true）
- `custom_ignore_patterns`：要忽略的附加文件模式（默认：[]）

#### 钩子设置 (`[hooks]`)

- `enabled`：启用 git 钩子集成（默认：false）
- `hook_types`：要安装的 git 钩子类型（默认：["prepare-commit-msg"]）

#### 提示设置 (`[prompts]`)

- `system_prompt`：定义 AI 行为的系统提示
- `user_prompt_template`：分析 diff 的模板（使用 `{diff}` 占位符）
- `simple_prompt_template`：简洁单行信息的模板

### 自定义 AI 提示

你可以通过编辑配置文件完全自定义 AI 生成提交信息的方式：

```bash
# 查看当前配置和文件位置
ai-commit config show

# 获取编辑提示的帮助
ai-commit config edit-prompts
```

编辑 `~/.config/ai-commit/config.toml` 来自定义提示：

````toml
[prompts]
system_prompt = """你是一个专注于清晰、简洁提交的高级开发者。
生成优先使用单行格式的常规提交信息。
只有在真正不相关的变更时才使用项目符号。"""

user_prompt_template = """为这些变更生成提交信息。
优先使用 72 字符以内的单行格式。

Git diff:
```diff
{diff}
````

只提供提交信息。"""

```

**自定义提示的技巧：**
- 在模板中保留 `{diff}` 占位符
- 使用 `ai-commit --dry-run` 测试更改
- 配置在下次运行时自动重新加载
- 在更新前备份自定义提示

## 提交信息格式

该工具生成遵循 Conventional Commits 规范的信息：

### 单行格式（首选）
用于专注的单一目的变更：
```

feat: 添加用户认证系统
fix: 解决数据库连接超时问题
refactor: 改进认证模块的错误处理

```

### 多行格式（用于复杂变更）
当存在以下情况时使用：
1. **多个不相关的功能变更**（一次提交中的不同功能/修复）
2. **单一功能的重大变更**，需要分解说明

```

feat: 添加用户管理和通知系统

- 实现带验证的用户 CRUD 操作
- 为用户事件添加邮件通知服务
- 创建用户管理的管理员仪表板

````

### 支持的类型
- `feat`：新功能
- `fix`：错误修复
- `docs`：仅文档变更
- `style`：代码样式变更（格式化等）
- `refactor`：既不修复错误也不添加功能的代码变更
- `perf`：性能改进
- `test`：添加或修正测试
- `chore`：构建过程或辅助工具变更

## 工作流示例

### 初始设置
```bash
# 一次性设置
export AI_COMMIT_ARK_API_KEY="your-api-key"
ai-commit config init

# 验证配置
ai-commit config show
````

### 标准工作流

```bash
# 对代码进行变更
git add .
ai-commit
# 审查生成的信息并确认
```

### 修订工作流

```bash
# 在最后一次提交后进行额外变更
git add .
ai-commit amend
# 审查合并变更的生成信息
```

### 预览变更

```bash
# 检查将生成什么信息而不提交
git add .
ai-commit --dry-run
```

### 自定义工作流

```bash
# 编辑配置文件
ai-commit config show  # 显示文件位置
# 编辑 ~/.config/ai-commit/config.toml

# 测试你的更改
git add .
ai-commit --dry-run
```

## 技术细节

### Diff 分析

- 分析 git diff 以理解代码变更
- 自动过滤锁文件和构建产物
- 考虑文件类型、变更模式和修改范围
- 支持暂存和未暂存变更分析

### AI 集成

- 使用先进的语言模型生成提交信息
- 发送上下文 diff 信息进行准确分析
- 遵守令牌限制和上下文窗口
- 优雅地处理 API 错误并提供回退信息
- 支持不同提交风格的完全可自定义提示

### 配置管理

- 符合 XDG Base Directory 规范（`~/.config/ai-commit/`）
- TOML 格式便于编辑和版本控制
- 支持 API 密钥的环境变量
- 如果配置缺失则回退到合理的默认值
- 配置变更的热重载，无需重启

### 安全性

- 与 GPG 签名的提交协作
- 遵守 git 配置设置
- 不在外部存储代码或敏感信息
- 仅通过环境变量管理 API 密钥
- 具有适当权限的本地配置文件

## 贡献

欢迎贡献！请随时：

- 通过 issues 提交错误报告和功能请求
- 为改进创建拉取请求
- 分享反馈和建议
- 帮助改进文档

### 开发设置

```bash
git clone <repository-url>
cd ai-commit
cargo test
cargo run -- --help
```

## 许可证

该项目采用 MIT 许可证。详见 LICENSE 文件。
