# 如何将此项目开源到 GitHub

## 📋 完整步骤

### 第一步：初始化本地 Git 仓库

以**管理员身份**打开 PowerShell 或 CMD，在项目根目录执行：

```bash
# 初始化 Git 仓库
git init

# 添加所有文件到暂存区
git add .

# 提交初始版本
git commit -m "Initial commit: SteamToolPlus v1.01"
```

---

### 第二步：在 GitHub 创建仓库

1. **访问** [GitHub](https://github.com/) 并登录账号
2. **点击右上角** `+` → `New repository`
3. **填写仓库信息**：
   - **Repository name**: `SteamToolPlus`（或其他你喜欢的名称）
   - **Description**: `Steam 游戏补丁注入工具 - 支持 Goldberg Emulator`
   - **Public**: ✅ 选中（公开仓库）
   - **Initialize this repository with**: ❌ **不要选中**（保持空白）
4. **点击** `Create repository`

---

### 第三步：关联远程仓库并推送

创建成功后，GitHub 会显示推送命令，执行：

```bash
# 添加远程仓库（替换为你的仓库地址）
git remote add origin https://github.com/你的用户名/SteamToolPlus.git

# 推送代码到 GitHub
git branch -M main
git push -u origin main
```

---

### 第四步：完善开源信息

#### 4.1 添加开源许可证（推荐）

在项目根目录创建 `LICENSE` 文件，选择适合的许可证：

**MIT License**（最宽松，推荐）：
```
MIT License

Copyright (c) 2026 [你的名字]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

或者访问 [GitHub 选择许可证](https://choosealicense.com/) 了解更多。

#### 4.2 添加贡献指南（可选）

创建 `CONTRIBUTING.md` 文件，说明如何贡献代码。

#### 4.3 添加代码规范（可选）

创建 `.editorconfig` 文件统一代码风格。

---

### 第五步：更新并提交

```bash
# 添加新文件
git add .

# 提交更改
git commit -m "Add LICENSE and open source documentation"

# 推送到 GitHub
git push
```

---

## ⚠️ 重要注意事项

### 1. 敏感信息检查

**在推送之前**，确保没有包含：
- ❌ 个人账号密码
- ❌ API Keys 或 Token
- ❌ 个人隐私信息
- ❌ 商业机密代码

### 2. 版权和合规性

由于此项目涉及游戏补丁工具，请注意：
- ⚠️ 明确说明仅供学习研究使用
- ⚠️ 不得用于商业用途
- ⚠️ 遵守当地法律法规
- ⚠️ 尊重游戏版权

建议在 README 中添加免责声明。

### 3. 大文件处理

如果项目包含大文件（>50MB），GitHub 会拒绝推送。使用 **Git LFS**：

```bash
# 安装 Git LFS
git lfs install

# 跟踪大文件类型
git lfs track "*.dll"
git lfs track "*.exe"

# 添加 .gitattributes 文件
git add .gitattributes
git commit -m "Configure Git LFS for binary files"
```

---

## 🔄 后续维护

### 更新代码到 GitHub

```bash
# 修改代码后
git add .
git commit -m "描述你的更改"
git push
```

### 查看状态

```bash
git status      # 查看文件状态
git log         # 查看提交历史
git remote -v   # 查看远程仓库
```

### 克隆仓库（其他人）

```bash
git clone https://github.com/你的用户名/SteamToolPlus.git
```

---

## 📚 有用的 Git 命令

```bash
# 撤销更改
git checkout -- <文件名>

# 撤销暂存
git reset HEAD <文件名>

# 修改最后一次提交
git commit --amend

# 查看差异
git diff

# 拉取远程更新
git pull
```

---

## 🎯 开源最佳实践

1. **清晰的 README**：说明项目用途、安装方法、使用示例
2. **选择合适的许可证**：保护你和使用者
3. **版本管理**：使用 Git Tags 标记版本号
4. **Issue 追踪**：及时响应用户反馈
5. **更新日志**：记录每次版本的变更
6. **代码审查**：接受 Pull Request 前仔细检查

---

## 🔗 相关资源

- [GitHub 官方文档](https://docs.github.com/)
- [Git 教程 - 廖雪峰](https://www.liaoxuefeng.com/wiki/896043488029600)
- [选择开源许可证](https://choosealicense.com/)
- [GitHub 学生包](https://education.github.com/pack)

---

**祝你开源顺利！** 🎉
