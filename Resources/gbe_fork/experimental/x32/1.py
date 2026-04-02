import tkinter as tk
from tkinter import ttk, filedialog, scrolledtext, messagebox
import os
import json
import shutil
import subprocess
import threading
import webbrowser
import psutil
import sys
import ctypes
import tempfile

class AutoSteamDownloader:
    def __init__(self, root):
        self.root = root
        self.root.title("Steam游戏本体下载工具 | 作者：鲸落_hi")
        self.root.geometry("1200x1000")
        self.root.protocol("WM_DELETE_WINDOW", self.on_window_close)

        # ====================== 核心适配：单文件/文件夹模式通用路径识别 ======================
        if getattr(sys, 'frozen', False) and hasattr(sys, '_MEIPASS'):
            # 单文件模式：程序运行目录是exe所在目录
            self.app_path = os.path.dirname(os.path.abspath(sys.executable))
            # 临时文件解压目录（PyInstaller自动管理，不用管）
            self.temp_path = sys._MEIPASS
        else:
            # 直接运行py文件的目录
            self.app_path = os.path.dirname(os.path.abspath(__file__))
            self.temp_path = self.app_path

        # 核心变量
        self.dd_exe_path = tk.StringVar()
        self.game_config_dir = tk.StringVar()
        self.save_dir = tk.StringVar()
        self.user_custom_path = False

        # 下载控制变量
        self.download_process = None
        self.is_paused = False
        self.is_downloading = False

        # 游戏数据变量
        self.game_appid = None
        self.game_name = None
        self.manifest_list = []

        # 创建界面
        self.create_top_links()
        self.create_main_ui()
        # 启动初始化
        self.app_start_init()

    # ====================== 启动初始化 ======================
    def app_start_init(self):
        self.log("="*50)
        self.log("🚀 程序启动，正在初始化...")
        self.log(f"📂 程序运行目录：{self.app_path}")

        # 自动识别同目录的ddv20.exe
        self.auto_detect_ddv20()
        # 初始化保存目录
        self.init_save_directory()

        self.log("✅ 初始化完成")
        self.log("="*50)

    # ====================== ddv20.exe自动识别 ======================
    def auto_detect_ddv20(self):
        ddv20_full_path = os.path.join(self.app_path, "ddv20.exe")
        if os.path.exists(ddv20_full_path) and os.path.isfile(ddv20_full_path):
            self.dd_exe_path.set(ddv20_full_path)
            self.log(f"✅ 自动识别到ddv20.exe")
        else:
            self.log("⚠️  未找到ddv20.exe，请手动选择")
            messagebox.showwarning("提示", "请将ddv20.exe和本程序放在同一个文件夹")

    # ====================== 保存目录三重兜底 ======================
    def init_save_directory(self):
        primary_path = r"D:\SteamGame"
        secondary_path = os.path.join(self.app_path, "SteamGame")
        
        # 获取用户文档目录（绝对兜底）
        try:
            CSIDL_PERSONAL = 5
            SHGFP_TYPE_CURRENT = 0
            buf = ctypes.create_unicode_buffer(ctypes.wintypes.MAX_PATH)
            ctypes.windll.shell32.SHGetFolderPathW(None, CSIDL_PERSONAL, None, SHGFP_TYPE_CURRENT, buf)
            doc_path = buf.value
            fallback_path = os.path.join(doc_path, "SteamGame")
        except:
            fallback_path = os.path.join(os.path.expanduser("~"), "SteamGame")

        # 按优先级检测
        for test_path in [primary_path, secondary_path, fallback_path]:
            try:
                os.makedirs(test_path, exist_ok=True)
                test_file = os.path.join(test_path, "write_test.tmp")
                with open(test_file, "w", encoding="utf-8") as f:
                    f.write("test")
                os.remove(test_file)
                self.base_save_path = test_path
                self.save_dir.set(test_path)
                self.log(f"📁 游戏保存目录：{test_path}")
                return
            except:
                continue

        messagebox.showerror("错误", "无法找到可用的保存目录，请以管理员身份运行")

    # ====================== 顶部链接 ======================
    def create_top_links(self):
        top_frame = ttk.Frame(self.root)
        top_frame.pack(fill=tk.X, padx=15, pady=8)

        left_frame = ttk.Frame(top_frame)
        left_frame.pack(side=tk.LEFT, anchor=tk.W)

        self.author_label = ttk.Label(
            left_frame,
            text="B站UP主：鲸落_hi",
            font=("微软雅黑", 14, "bold"),
            foreground="#FF66B2"
        )
        self.author_label.pack(side=tk.LEFT, padx=0)

        self.bilibili_link = ttk.Label(
            left_frame,
            text="  跳转B站主页",
            font=("微软雅黑", 14, "underline"),
            foreground="#FF66B2",
            cursor="hand2"
        )
        self.bilibili_link.pack(side=tk.LEFT, padx=10)
        self.bilibili_link.bind("<Button-1>", lambda e: webbrowser.open("https://space.bilibili.com/405707676"))

        right_frame = ttk.Frame(top_frame)
        right_frame.pack(side=tk.RIGHT, anchor=tk.E)

        self.manifesthub_link = ttk.Label(
            right_frame,
            text="ManifestHub清单下载",
            font=("微软雅黑", 12, "underline"),
            foreground="#FF66B2",
            cursor="hand2"
        )
        self.manifesthub_link.pack(side=tk.RIGHT, padx=5)
        self.manifesthub_link.bind("<Button-1>", lambda e: webbrowser.open("https://github.com/SteamAutoCracks/ManifestHub"))

    # ====================== 主界面 ======================
    def create_main_ui(self):
        path_frame = ttk.LabelFrame(self.root, text="第一步：路径配置", padding=12)
        path_frame.pack(fill=tk.X, padx=12, pady=8)

        ttk.Label(path_frame, text="1. 选择ddv20.exe程序：").grid(row=0, column=0, sticky=tk.W, pady=8)
        ttk.Entry(path_frame, textvariable=self.dd_exe_path, width=72).grid(row=0, column=1, padx=10)
        ttk.Button(path_frame, text="浏览选择", command=self.select_dd_exe).grid(row=0, column=2)

        ttk.Label(path_frame, text="2. 选择游戏配置文件夹：").grid(row=1, column=0, sticky=tk.W, pady=8)
        ttk.Entry(path_frame, textvariable=self.game_config_dir, width=72).grid(row=1, column=1, padx=10)
        ttk.Button(path_frame, text="浏览选择", command=self.auto_parse_game_folder).grid(row=1, column=2)

        ttk.Label(path_frame, text="3. 游戏保存目录：").grid(row=2, column=0, sticky=tk.W, pady=8)
        ttk.Entry(path_frame, textvariable=self.save_dir, width=72).grid(row=2, column=1, padx=10)
        ttk.Button(path_frame, text="浏览选择", command=self.select_save_dir).grid(row=2, column=2)

        self.info_frame = ttk.LabelFrame(self.root, text="第二步：游戏信息确认", padding=12)
        self.info_frame.pack(fill=tk.X, padx=12, pady=8)

        self.game_info_label = ttk.Label(
            self.info_frame,
            text="游戏名称：未加载 | AppID：未加载 | 可下载Depot数量：0个",
            font=("微软雅黑", 10, "bold")
        )
        self.game_info_label.pack(anchor=tk.W)

        btn_frame = ttk.Frame(self.root)
        btn_frame.pack(pady=15)

        self.download_btn = ttk.Button(
            btn_frame,
            text="开始下载",
            command=self.start_download_thread,
            state=tk.DISABLED,
            width=15
        )
        self.download_btn.pack(side=tk.LEFT, padx=10, ipadx=10, ipady=6)

        self.pause_btn = ttk.Button(
            btn_frame,
            text="暂停下载",
            command=self.pause_or_resume_download,
            state=tk.DISABLED,
            width=15
        )
        self.pause_btn.pack(side=tk.LEFT, padx=10, ipadx=10, ipady=6)

        log_frame = ttk.LabelFrame(self.root, text="下载日志", padding=10)
        log_frame.pack(fill=tk.BOTH, expand=True, padx=12, pady=8)

        self.log_text = scrolledtext.ScrolledText(log_frame, state="disabled", height=20)
        self.log_text.pack(fill=tk.BOTH, expand=True)

    # ====================== 基础方法 ======================
    def log(self, msg):
        self.log_text.config(state="normal")
        try:
            self.log_text.insert(tk.END, f"{msg}\n")
        except:
            self.log_text.insert(tk.END, f"{msg.encode('gbk', errors='ignore').decode('gbk', errors='ignore')}\n")
        self.log_text.see(tk.END)
        self.log_text.config(state="disabled")

    def select_dd_exe(self):
        file_path = filedialog.askopenfilename(
            title="选择ddv20.exe",
            filetypes=[("可执行程序", "ddv20.exe"), ("所有EXE文件", "*.exe")]
        )
        if file_path:
            self.dd_exe_path.set(file_path)
            self.log(f"✅ 已选择ddv20.exe")

    def select_save_dir(self):
        dir_path = filedialog.askdirectory(title="选择游戏保存目录")
        if dir_path:
            self.user_custom_path = True
            self.save_dir.set(dir_path)
            self.log(f"✅ 已自定义保存目录")
            os.makedirs(dir_path, exist_ok=True)

    # ====================== 游戏配置解析 ======================
    def auto_parse_game_folder(self):
        folder_path = filedialog.askdirectory(title="选择包含json/manifest/key.vdf的游戏文件夹")
        if folder_path:
            self.auto_parse_game_folder_by_path(folder_path)

    def auto_parse_game_folder_by_path(self, folder_path):
        self.game_config_dir.set(folder_path)
        self.log("="*50)
        self.log(f"📂 已选择游戏配置文件夹")
        self.log("🔍 开始解析...")

        self.game_appid = None
        self.game_name = None
        self.manifest_list = []

        try:
            json_files = [f for f in os.listdir(folder_path) if f.lower().endswith(".json")]
            if not json_files:
                self.log("❌ 错误：未找到.json文件")
                messagebox.showerror("错误", "未找到.json配置文件")
                return

            json_path = os.path.join(folder_path, json_files[0])
            try:
                with open(json_path, "r", encoding="utf-8") as f:
                    game_json = json.load(f)
            except:
                with open(json_path, "r", encoding="gbk") as f:
                    game_json = json.load(f)

            self.game_appid = game_json.get("appid")
            self.game_name = game_json.get("schinese_name", game_json.get("name", "未知游戏"))
            self.log(f"✅ 解析成功：{self.game_name} (AppID: {self.game_appid})")

            if not self.user_custom_path:
                game_save_path = os.path.join(self.base_save_path, str(self.game_appid))
                self.save_dir.set(game_save_path)
                os.makedirs(game_save_path, exist_ok=True)
                self.log(f"📁 已自动设置保存路径")

            self.manifest_list = [f for f in os.listdir(folder_path) if f.lower().endswith(".manifest")]
            if not self.manifest_list:
                self.log("❌ 错误：未找到.manifest文件")
                messagebox.showerror("错误", "未找到.manifest清单文件")
                return
            self.log(f"📦 找到 {len(self.manifest_list)} 个Depot清单")

            key_vdf_path = os.path.join(folder_path, "key.vdf")
            config_vdf_path = os.path.join(folder_path, "config.vdf")
            if not os.path.exists(key_vdf_path):
                self.log("❌ 错误：未找到key.vdf")
                messagebox.showerror("错误", "未找到key.vdf文件")
                return

            shutil.copy2(key_vdf_path, config_vdf_path)
            self.log("✅ 已生成config.vdf")

            self.game_info_label.config(
                text=f"游戏名称：{self.game_name} | AppID：{self.game_appid} | 可下载Depot数量：{len(self.manifest_list)}个"
            )
            self.download_btn.config(state=tk.NORMAL)
            self.log("🎉 解析完成，可开始下载")
            self.log("="*50)

        except Exception as e:
            self.log(f"❌ 解析失败：{str(e)}")
            messagebox.showerror("错误", f"解析失败：{str(e)}")

    # ====================== 下载控制 ======================
    def pause_or_resume_download(self):
        if not self.download_process or self.download_process.poll() is not None:
            messagebox.showwarning("提示", "当前没有正在运行的下载任务")
            return

        try:
            process = psutil.Process(self.download_process.pid)
            if not self.is_paused:
                process.suspend()
                self.is_paused = True
                self.pause_btn.config(text="继续下载")
                self.log("⚠️  下载已暂停")
            else:
                process.resume()
                self.is_paused = False
                self.pause_btn.config(text="暂停下载")
                self.log("▶️  下载已恢复")
        except Exception as e:
            self.log(f"❌ 操作失败：{str(e)}")
            messagebox.showerror("错误", f"操作失败：{str(e)}")

    def start_download_thread(self):
        if not os.path.exists(self.dd_exe_path.get()):
            messagebox.showerror("错误", "请先选择ddv20.exe")
            return
        if not os.path.exists(self.game_config_dir.get()):
            messagebox.showerror("错误", "游戏配置文件夹不存在")
            return
        try:
            os.makedirs(self.save_dir.get(), exist_ok=True)
        except:
            messagebox.showerror("错误", "保存目录无写入权限")
            return

        self.is_paused = False
        self.is_downloading = True
        threading.Thread(target=self.run_download, daemon=True).start()

    def run_download(self):
        self.download_btn.config(state=tk.DISABLED, text="正在下载中...")
        self.pause_btn.config(state=tk.NORMAL)
        self.log("🚀 开始下载...")

        download_cmd = [
            self.dd_exe_path.get(),
            "-lu", "China",
            "--use-http",
            "-o", self.save_dir.get(),
            "app",
            "-p", self.game_config_dir.get()
        ]

        self.log("-"*50)

        try:
            self.download_process = subprocess.Popen(
                download_cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                encoding="gbk",
                errors="ignore",
                cwd=self.app_path
            )

            for line in self.download_process.stdout:
                self.log(line.strip())

            self.download_process.wait()
            self.log("-"*50)

            if self.download_process.returncode == 0:
                self.log(f"✅ 下载完成！保存路径：{self.save_dir.get()}")
                messagebox.showinfo("完成", f"下载完成！\n保存路径：{self.save_dir.get()}")
            else:
                self.log(f"⚠️  下载结束，退出码：{self.download_process.returncode}")
                messagebox.showwarning("提示", f"下载结束，退出码：{self.download_process.returncode}")

        except Exception as e:
            self.log(f"❌ 下载失败：{str(e)}")
            messagebox.showerror("错误", f"下载失败：{str(e)}")

        finally:
            self.is_downloading = False
            self.is_paused = False
            self.download_process = None
            self.download_btn.config(state=tk.NORMAL, text="开始下载")
            self.pause_btn.config(state=tk.DISABLED, text="暂停下载")

    # ====================== 窗口关闭 ======================
    def on_window_close(self):
        if self.download_process and self.download_process.poll() is None:
            if messagebox.askokcancel("退出确认", "当前有正在下载的任务，退出将终止下载，是否继续？"):
                try:
                    parent = psutil.Process(self.download_process.pid)
                    for child in parent.children(recursive=True):
                        child.kill()
                    parent.kill()
                except:
                    pass
                self.root.destroy()
        else:
            self.root.destroy()

if __name__ == "__main__":
    try:
        ctypes.windll.shcore.SetProcessDpiAwareness(1)
    except:
        try:
            ctypes.windll.user32.SetProcessDPIAware()
        except:
            pass
    root = tk.Tk()
    app = AutoSteamDownloader(root)
    root.mainloop()