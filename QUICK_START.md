# 🚀 快速开源操作指南

## 一分钟快速开始

### 1️⃣ 初始化 Git（在項目根目錄執行）

```powershell
git init
git add .
git commit -m "Initial commit: SteamToolPlus v1.01"
```

### 2️⃣ 在 GitHub 創建倉庫

1. 訪問 https://github.com
2. 點擊右上角 `+` → `New repository`
3. 填寫倉庫名稱：`SteamToolPlus`
4. 選擇 `Public`（公開）
5. **不要**勾選 "Initialize this repository with"
6. 點擊 `Create repository`

### 3️⃣ 推送代碼到 GitHub

```powershell
# 替換為你的 GitHub 用戶名
git remote add origin https://github.com/你的用户名/SteamToolPlus.git

git branch -M main
git push -u origin main
```

完成！🎉 你的項目已經開源了！

---

## ⚠️ 推送前必讀

### 檢查清單

- [ ] 已創建 `.gitignore` 文件（排除 bin/、obj/、.vs/）
- [ ] 已創建 `LICENSE` 文件（MIT 許可證）
- [ ] 已創建 `.gitattributes` 文件（處理大文件）
- [ ] README.md 包含免責聲明
- [ ] 沒有包含個人敏感信息

### 大文件處理

如果項目包含大於 50MB 的文件，需要先安裝 Git LFS：

```powershell
# 安裝 Git LFS
git lfs install

# 重新添加文件
git add .
git commit -m "Configure Git LFS"
git push
```

---

## 🔄 後續更新代碼

```powershell
# 修改代碼後
git add .
git commit -m "描述你的更改"
git push
```

---

## 📊 查看狀態

```powershell
git status      # 查看文件狀態
git log         # 查看提交歷史
git remote -v   # 查看遠程倉庫
```

---

## 🔗 相關資源

- [GitHub 官方文檔](https://docs.github.com/)
- [Git 教程](https://www.liaoxuefeng.com/wiki/896043488029600)
- [選擇開源許可證](https://choosealicense.com/)

---

**祝你開源順利！** ✨
