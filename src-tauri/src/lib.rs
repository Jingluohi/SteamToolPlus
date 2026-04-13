// Steam Tool Plus - Tauri 后端库
// 纯本地运行，无网络请求

use std::path::{Path, PathBuf};
use tauri::AppHandle;

// ==================== 游戏配置数据结构 ====================

/// 游戏标签
#[derive(Debug, Clone, serde::Serialize)]
pub struct GameTag {
    pub patch_type: u8,
    pub patch_source_path: &'static str,
    pub download_url: Option<&'static str>,
}

/// 游戏配置
#[derive(Debug, Clone, serde::Serialize)]
pub struct GameConfig {
    pub game_id: &'static str,
    pub game_name: &'static str,
    pub chinese_name: &'static str,
    pub downloadable: bool,
    pub tags: &'static [GameTag],
}

/// 嵌入的游戏配置数据
/// 添加新游戏时，在此数组中添加新的 GameConfig
pub static GAMES_CONFIG: &[GameConfig] = &[
    // 生化危机：安魂曲
    GameConfig {
        game_id: "3764200",
        game_name: "Resident Evil Requiem",
        chinese_name: "生化危机：安魂曲",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/3764200",
                download_url: Some("https://pan.baidu.com/s/1gBvS2UjvVBM5msWSHE45EQ?pwd=93su"),
            },
        ],
    },
    // 红色沙漠
    GameConfig {
        game_id: "3321460",
        game_name: "Crimson Desert",
        chinese_name: "红色沙漠",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/3321460",
                download_url: Some("https://pan.baidu.com/s/12Q3yy5cHjoO5rODBRXRY1Q?pwd=v51p"),
            },
            GameTag {
                patch_type: 3,
                patch_source_path: "Resources/crack/D_加密虚拟机/3321460",
                download_url: Some("https://pan.baidu.com/s/1fVilu8rcKiAsL-FHUHfLZA?pwd=a4vw"),
            },
        ],
    },
    // 黑神话：悟空
    GameConfig {
        game_id: "2358720",
        game_name: "Black Myth: Wukong",
        chinese_name: "黑神话：悟空",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 3,
                patch_source_path: "Resources/crack/D_加密虚拟机/2358720",
                download_url: Some("https://pan.baidu.com/s/1lQwDDi_vEItrV5aCcCaLcw?pwd=dbwj"),
            },
        ],
    },
    // 剑星
    GameConfig {
        game_id: "3489700",
        game_name: "Stellar Blade",
        chinese_name: "剑星",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 3,
                patch_source_path: "Resources/crack/D_加密虚拟机/3489700",
                download_url: Some("https://pan.baidu.com/s/1jIZRjAseMlQZFoh9KmTgfQ?pwd=anvn"),
            },
        ],
    },
    // 怪物猎人物语 3：命运双龙
    GameConfig {
        game_id: "2852190",
        game_name: "Monster Hunter Stories 3: Twisted Reflection",
        chinese_name: "怪物猎人物语 3：命运双龙",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 3,
                patch_source_path: "Resources/crack/D_加密虚拟机/2852190",
                download_url: Some("https://pan.baidu.com/s/1R1hUpnDwQopeN8FOVutKHA?pwd=548s"),
            },
        ],
    },
    // 怪物猎人：荒野
    GameConfig {
        game_id: "2246340",
        game_name: "Monster Hunter Wilds",
        chinese_name: "怪物猎人：荒野",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 3,
                patch_source_path: "Resources/crack/D_加密虚拟机/2246340",
                download_url: Some("https://pan.baidu.com/s/1u2UzeqfbGr1eG1dt-k4M8g?pwd=tyz6"),
            },
        ],
    },
    // 怪物猎人：世界
    GameConfig {
        game_id: "582010",
        game_name: "Monster Hunter: World",
        chinese_name: "怪物猎人：世界",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 1,
                patch_source_path: "Resources/crack/局域网联机/582010",
                download_url: Some("https://pan.baidu.com/s/18fq70UxJalUmiW7ZaWnaKA?pwd=c9tg"),
            },
        ],
    },
    // 怪物猎人：崛起
    GameConfig {
        game_id: "1446780",
        game_name: "MONSTER HUNTER RISE",
        chinese_name: "怪物猎人：崛起",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1446780",
                download_url: Some("https://pan.baidu.com/s/18fq70UxJalUmiW7ZaWnaKA?pwd=c9tg"),
            },
        ],
    },
    // 生化危机 4
    GameConfig {
        game_id: "2050650",
        game_name: "Resident Evil 4",
        chinese_name: "生化危机 4",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/2050650",
                download_url: Some("https://pan.baidu.com/s/1aiwhmInSJxkH0e3yzo2KKw?pwd=61gz"),
            },
        ],
    },
    // 生化危机7
    GameConfig {
        game_id: "418370",
        game_name: "Resident Evil 7 Biohazard",
        chinese_name: "生化危机7",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/418370",
                download_url: None,
            },
        ],
    },
    // 生化危机8
    GameConfig {
        game_id: "1196590",
        game_name: "Resident Evil Village",
        chinese_name: "生化危机8",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/1196590",
                download_url: None,
            },
        ],
    },
    // 艾尔登法环
    GameConfig {
        game_id: "1245620",
        game_name: "ELDEN RING",
        chinese_name: "艾尔登法环",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 1,
                patch_source_path: "Resources/crack/局域网联机/1245620",
                download_url: Some("https://pan.baidu.com/s/11id9KEHy6DKfsu1T8tv3Kw?pwd=e7rr"),
            },
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1245620",
                download_url: Some("https://pan.baidu.com/s/1jFfTmCbbUFcAWqbwgEe3ew?pwd=rnen"),
            },
        ],
    },
    // 极限竞速：地平线 5
    GameConfig {
        game_id: "1551360",
        game_name: "Forza Horizon 5",
        chinese_name: "极限竞速：地平线 5",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1551360",
                download_url: Some("https://pan.baidu.com/s/1g3LYKCNMQJX1vjiqrP0MaQ?pwd=fhf4"),
            },
        ],
    },
    // 微软模拟飞行2020
    GameConfig {
        game_id: "1250410",
        game_name: "MicrosoftFlightSimulator2020",
        chinese_name: "微软模拟飞行2020",
        downloadable: false,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1250410",
                download_url: Some("https://pan.baidu.com/s/1kmKFkGnBq_MJK4fDfTtD9A?pwd=drau"),
            },
        ],
    },
    // 仁王 3
    GameConfig {
        game_id: "3681010",
        game_name: "Nioh 3",
        chinese_name: "仁王 3",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/3681010",
                download_url: Some("https://pan.baidu.com/s/1TSTO8U2XsEfNqpQirijepw?pwd=rkej"),
            },
        ],
    },

    // 杀戮尖塔 2
    GameConfig {
        game_id: "2868840",
        game_name: "Slay the Spire 2",
        chinese_name: "杀戮尖塔 2",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/2868840",
                download_url: Some("https://pan.baidu.com/s/1qOx-omcoXkhfe8v16IhOSg?pwd=hpu9"),
            },
        ],
    },
    // 艾尔登法环 黑夜君临
    GameConfig {
        game_id: "2622380",
        game_name: "ELDEN RING NIGHTREIGN",
        chinese_name: "艾尔登法环 黑夜君临",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 1,
                patch_source_path: "Resources/crack/局域网联机/2622380",
                download_url: Some("https://pan.baidu.com/s/1V0DPlV7w5_mReZYX6FqOAw?pwd=qg3c"),
            },
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/2622380",
                download_url: Some("https://pan.baidu.com/s/1XxAz_Ae05Zoso99wOI4Y5g?pwd=hrhp"),
            },
        ],
    },
    // 木筏求生
    GameConfig {
        game_id: "648800",
        game_name: "Raft",
        chinese_name: "木筏求生",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/648800",
                download_url: Some("https://pan.baidu.com/s/1FUsmbrJHP8poRtzUHmD0xg?pwd=yb6u"),
            },
        ],
    },
    // 幻兽帕鲁
    GameConfig {
        game_id: "1623730",
        game_name: "Palworld",
        chinese_name: "幻兽帕鲁",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1623730",
                download_url: Some("https://pan.baidu.com/s/18lD8KX2HRMZta6QRRSq07Q?pwd=hmww"),
            },
        ],
    },
    // 消逝的光芒：困兽
    GameConfig {
        game_id: "3008130",
        game_name: "Dying Light: The Beast Restored Land",
        chinese_name: "消逝的光芒：困兽",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/3008130",
                download_url: Some("https://pan.baidu.com/s/1qhbwdPOgBZJdDbdzAzmy3w?pwd=t1jy"),
            },
        ],
    },
    // 空洞骑士：丝之歌
    GameConfig {
        game_id: "1030300",
        game_name: "Hollow Knight: Silksong",
        chinese_name: "空洞骑士：丝之歌",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1030300",
                download_url: Some("https://pan.baidu.com/s/1-RGnHha4Wv2inbB2oWM6wg?pwd=mnak"),
            },
        ],
    },
    // 黑暗之魂 3
    GameConfig {
        game_id: "374320",
        game_name: "DARK SOULS III",
        chinese_name: "黑暗之魂 3",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/374320",
                download_url: Some("https://pan.baidu.com/s/1l7O7IhjHvzxe0PGvKlBo4g?pwd=b558"),
            },
        ],
    },
    // Ghost of Tsushima DIRECTOR'S CUT
    GameConfig {
        game_id: "2215430",
        game_name: "Ghost of Tsushima DIRECTOR'S CUT",
        chinese_name: "对马岛之魂：导演剪辑版",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/2215430",
                download_url: Some("https://pan.baidu.com/s/1g3nX6fnaIzaIXtqeTeZ8Ww?pwd=rkhw"),
            },
        ],
    },
    // PEAK
    GameConfig {
        game_id: "3527290",
        game_name: "PEAK",
        chinese_name: "PEAK",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/3527290",
                download_url: Some("https://pan.baidu.com/s/112QwFnJRuv2WBD8gcI17BQ?pwd=5afj"),
            },
        ],
    },
    // 逃脱密室
    GameConfig {
        game_id: "1943950",
        game_name: "Escape the Backrooms",
        chinese_name: "逃脱密室",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1943950",
                download_url: Some("https://pan.baidu.com/s/17krJ-KYXJLU6n0Pb8bUQRQ?pwd=vewf"),
            },
        ],
    },
    // 森林之子
    GameConfig {
        game_id: "1326470",
        game_name: "Sons of the Forest",
        chinese_name: "森林之子",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1326470",
                download_url: Some("https://pan.baidu.com/s/1-RUrC-LzLfDPl1DooK6D4g?pwd=udrz"),
            },
        ],
    },
    // 致命公司
    GameConfig {
        game_id: "1966720",
        game_name: "Lethal Company",
        chinese_name: "致命公司",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1966720",
                download_url: Some("https://pan.baidu.com/s/1Wy3ty8dSzoRtC-AFpYuJDw?pwd=ef17"),
            },
            GameTag {
                patch_type: 1,
                patch_source_path: "Resources/crack/局域网联机/1966720",
                download_url: Some("https://pan.baidu.com/s/1oMGA2-PRJ9jH8CzfqR6NjQ?pwd=kbjy"),
            },
        ],
    },
    // 死亡细胞
    GameConfig {
        game_id: "588650",
        game_name: "Dead Cells",
        chinese_name: "死亡细胞",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/588650",
                download_url: Some("https://pan.baidu.com/s/1z8ZaIhzQ7SLSCqD3ZRw4Qg?pwd=bqqi"),
            },
        ],
    },
    // 严阵以待
    GameConfig {
        game_id: "1144200",
        game_name: "Ready or Not",
        chinese_name: "严阵以待",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1144200",
                download_url: Some("https://pan.baidu.com/s/1ji4KSzhHQHD1wkYazzebJA?pwd=di38"),
            },
        ],
    },
    // 梦之形
    GameConfig {
        game_id: "2444750",
        game_name: "Shape of Dreams",
        chinese_name: "梦之形",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/2444750",
                download_url: Some("https://pan.baidu.com/s/1j4RQy36-_Fe7vBzDkbRVTw?pwd=euc"),
            },
        ],
    },
    // 死亡搁浅2
    GameConfig {
        game_id: "3280350",
        game_name: "DEATH STRANDING 2: ON THE BEACH",
        chinese_name: "死亡搁浅2",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/3280350",
                download_url: Some("https://pan.baidu.com/s/1GVboGSSWwddECocYkONNAA?pwd=9edu"),
            },
        ],
    },
    // 光与影：33号远征队
    GameConfig {
        game_id: "1903340",
        game_name: "Clair Obscur: Expedition 33",
        chinese_name: "光与影：33号远征队",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/1903340",
                download_url: Some("https://pan.baidu.com/s/1ZppuNFLAai8NbckHLVS9og?pwd=79p2"),
            },
        ],
    },
    // 骑马与砍杀2霸主
    GameConfig {
        game_id: "261550",
        game_name: "Mount & Blade II: Bannerlord",
        chinese_name: "骑马与砍杀2霸主",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/261550",
                download_url: Some("https://pan.baidu.com/s/1c9nc8sRzD8m0OR0of8aTiQ?pwd=xejd"),
            },
        ],
    },
    // 夜族崛起
    GameConfig {
        game_id: "1604030",
        game_name: "V Rising",
        chinese_name: "夜族崛起",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1604030",
                download_url: Some("https://pan.baidu.com/s/1x1zKqcSbhfyh3PwHnOYI_w?pwd=prai"),
            },
        ],
    },
    // GTFO
    GameConfig {
        game_id: "493520",
        game_name: "GTFO",
        chinese_name: "GTFO",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/493520",
                download_url: Some("https://pan.baidu.com/s/1ePYMc4Bptc9-aAoy0UtPCA?pwd=t75d"),
            },
        ],
    },
    // 吸血鬼幸存者
    GameConfig {
        game_id: "1794680",
        game_name: "Vampire Survivors",
        chinese_name: "吸血鬼幸存者",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/1794680",
                download_url: Some("https://pan.baidu.com/s/1_Wzgexc_RMycvoj8FSAnoQ?pwd=phqb"),
            },
        ],
    },
    // 雾所王国
    GameConfig {
        game_id: "1203620",
        game_name: "Enshrouded",
        chinese_name: "雾所王国",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1203620",
                download_url: Some("https://pan.baidu.com/s/1xjZ_a2P_LP17rl1RCpzIZQ?pwd=hbdk"),
            },
        ],
    },
    // 明末：渊虚之羽
    GameConfig {
        game_id: "2277560",
        game_name: "WUCHANG: Fallen Feathers",
        chinese_name: "明末：渊虚之羽",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/2277560",
                download_url: Some("https://pan.baidu.com/s/1yIcwrWC0TuSNX48EsDlp-w?pwd=3eig"),
            },
        ],
    },
    
    // 博德之门3
    GameConfig {
        game_id: "1086940",
        game_name: "Baldur's Gate 3",
        chinese_name: "博德之门3",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1086940",
                download_url: Some("https://pan.baidu.com/s/1bK7bCGq5YAx8G69yDC98OA?pwd=e8qj"),
            },
        ],
    },
    // 卧龙：苍天陨落
    GameConfig {
        game_id: "1448440",
        game_name: "Wo Long: Fallen Dynasty",
        chinese_name: "卧龙：苍天陨落",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1448440",
                download_url: Some("https://pan.baidu.com/s/1UgQuhmj3UC_r72vgjnMZOg?pwd=f1ex"),
            },
        ],
    },

    // 寂静岭f
    GameConfig {
        game_id: "2947440",
        game_name: "SILENT HILL f",
        chinese_name: "寂静岭f",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 0,
                patch_source_path: "Resources/crack/免_steam/2947440",
                download_url: Some("https://pan.baidu.com/s/1sO31AV4I3fmcY3M6bu6ygg?pwd=2qw3"),
            },
        ],
    },
    // 失落城堡
    GameConfig {
        game_id: "434650",
        game_name: "Lost Castle",
        chinese_name: "失落城堡",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/434650",
                download_url: Some("https://pan.baidu.com/s/1YiktCwEpH-cdc5NIUO8nOg?pwd=n7nc"),
            },
        ],
    },
    // 赛博朋克2077
    GameConfig {
        game_id: "1091500",
        game_name: "Cyberpunk 2077",
        chinese_name: "赛博朋克2077",
        downloadable: true,
        tags: &[],
    },
    // 绝对魔杖
    GameConfig {
        game_id: "1904480",
        game_name: "Absolum",
        chinese_name: "绝对魔杖",
        downloadable: true,
        tags: &[
            GameTag {
                patch_type: 2,
                patch_source_path: "Resources/crack/steam_联机/1904480",
                download_url: Some("https://pan.baidu.com/s/1v52dEXP8v_Cx6pBLQ-274w?pwd=99w8"),
            },
        ],
    },
];

// ==================== 游戏配置数据结构结束 ====================

/// 获取资源目录路径
/// 在开发模式下使用项目根目录的 resources 文件夹
/// 在发布模式下使用程序同目录下的 resources 文件夹
fn get_resource_dir(_app: &AppHandle) -> Result<PathBuf, String> {
    // 获取程序所在目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("无法获取程序路径: {}", e))?
        .parent()
        .ok_or("无法获取程序所在目录")?
        .to_path_buf();
    
    // 首先检查程序同目录下的 resources 文件夹
    let resources_path = exe_dir.join("resources");
    if resources_path.exists() && resources_path.is_dir() {
        return Ok(resources_path);
    }
    
    // 检查程序所在目录是否直接包含资源（crack、ddv20.exe等）
    if exe_dir.join("crack").exists() || exe_dir.join("ddv20.exe").exists() {
        return Ok(exe_dir);
    }
    
    // 开发模式：尝试向上级目录查找（target/debug 或 target/release -> 项目根目录）
    // exe_dir 可能是 target/debug 或 target/release
    let parent = exe_dir.parent(); // target
    if let Some(target_dir) = parent {
        let project_resources = target_dir.parent().unwrap_or(target_dir).join("resources");
        if project_resources.exists() && project_resources.is_dir() {
            return Ok(project_resources);
        }
    }
    
    Err("无法找到资源目录".to_string())
}

/// 应用程序状态
pub struct AppState {
    // 可以在这里添加应用级别的状态
}

impl Default for AppState {
    fn default() -> Self {
        Self {}
    }
}

/// 初始化应用程序
fn setup_app(_app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 窗口配置已在 tauri.conf.json 中完成
    // Mica 效果通过前端 CSS 实现

    // 插件已在 Builder 中初始化，这里不需要重复初始化

    Ok(())
}

/// 读取清单文件夹内容
/// 返回文件夹中的 .json、.vdf 和 .manifest 文件列表
#[tauri::command]
async fn read_manifest_folder(folder_path: String) -> Result<ManifestFolderResult, String> {
    let path = Path::new(&folder_path);

    if !path.exists() {
        return Err("文件夹不存在".to_string());
    }

    if !path.is_dir() {
        return Err("路径不是文件夹".to_string());
    }

    let mut json_files = Vec::new();
    let mut vdf_files = Vec::new();
    let mut manifest_files = Vec::new();

    // 读取目录内容
    let entries = std::fs::read_dir(path).map_err(|e| format!("读取目录失败: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
        let file_path = entry.path();

        if file_path.is_file() {
            if let Some(ext) = file_path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                let path_str = file_path.to_string_lossy().to_string();

                match ext.as_str() {
                    "json" => json_files.push(path_str),
                    "vdf" => vdf_files.push(path_str),
                    "manifest" => manifest_files.push(path_str),
                    _ => {}
                }
            }
        }
    }

    Ok(ManifestFolderResult {
        json_files,
        vdf_files,
        manifest_files,
    })
}

/// 清单文件夹读取结果
#[derive(serde::Serialize)]
pub struct ManifestFolderResult {
    #[serde(rename = "jsonFiles")]
    json_files: Vec<String>,
    #[serde(rename = "vdfFiles")]
    vdf_files: Vec<String>,
    #[serde(rename = "manifestFiles")]
    manifest_files: Vec<String>,
}

/// 读取文本文件内容
#[tauri::command]
async fn read_text_file(file_path: String) -> Result<String, String> {
    let path = Path::new(&file_path);

    if !path.exists() {
        return Err("文件不存在".to_string());
    }

    std::fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))
}

/// 读取 games_config.json 文件
/// 读取游戏配置数据
/// 数据已嵌入在程序中，无需读取外部文件
#[tauri::command]
async fn read_games_config() -> Result<String, String> {
    // 将嵌入的 GAMES_CONFIG 数据序列化为 JSON 字符串
    serde_json::to_string(GAMES_CONFIG)
        .map_err(|e| format!("序列化游戏配置失败: {}", e))
}

/// 获取可用的游戏盘符
/// 优先返回D盘，如果D盘不存在则返回C盘
#[tauri::command]
async fn get_available_drive() -> Result<String, String> {
    // 检查D盘是否存在
    let d_drive = Path::new("D:\\");
    if d_drive.exists() {
        Ok("D:".to_string())
    } else {
        // 默认返回C盘
        Ok("C:".to_string())
    }
}

/// 获取游戏的清单文件夹路径
/// 自动查找 resources/manifest/游戏ID 目录
/// 如果找到则返回完整路径，否则返回空字符串
#[tauri::command]
async fn get_manifest_path(app: AppHandle, game_id: String) -> Result<String, String> {
    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;

    // 构建清单文件夹路径：resources/manifest/游戏ID
    let manifest_dir = resource_dir.join("manifest").join(&game_id);

    // 检查文件夹是否存在
    if manifest_dir.exists() && manifest_dir.is_dir() {
        Ok(manifest_dir.to_string_lossy().to_string())
    } else {
        // 未找到清单文件夹，返回空字符串
        Ok("".to_string())
    }
}

/// 启动游戏下载
/// 调用 ddv20.exe 进行下载，并在新终端窗口中运行
#[tauri::command]
async fn start_game_download(
    app: AppHandle,
    manifest_path: String,
    download_path: String,
) -> Result<DownloadResult, String> {
    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;

    // 构建 ddv20.exe 的完整路径
    let ddv20_path = resource_dir.join("ddv20.exe");

    if !ddv20_path.exists() {
        return Err(format!("ddv20.exe 不存在: {}", ddv20_path.display()));
    }

    // 检查下载路径的父目录是否存在，不存在则创建
    let download_dir = Path::new(&download_path);
    if let Some(parent) = download_dir.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("创建下载目录失败: {}", e))?;
        }
    }

    // 构建命令参数
    // ddv20.exe -lu China --use-http -o "游戏保存文件夹" app -p "清单文件夹"
    let _args = vec![
        "-lu".to_string(),
        "China".to_string(),
        "--use-http".to_string(),
        "-o".to_string(),
        download_path.clone(),
        "app".to_string(),
        "-p".to_string(),
        manifest_path.clone(),
    ];

    // 在 Windows 上使用 start 命令在新终端窗口中运行
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        // 将路径中的反斜杠替换为正斜杠，避免转义问题
        let ddv20_path_str = ddv20_path.to_string_lossy().replace('\\', "/");
        let download_path_str = download_path.replace('\\', "/");
        let manifest_path_str = manifest_path.replace('\\', "/");

        // 使用 start 命令在新窗口中运行 ddv20
        // start "" "ddv20.exe路径" -lu China --use-http -o "下载路径" app -p "清单路径"
        let child = Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg("")
            .arg(&ddv20_path_str)
            .arg("-lu")
            .arg("China")
            .arg("--use-http")
            .arg("-o")
            .arg(&download_path_str)
            .arg("app")
            .arg("-p")
            .arg(&manifest_path_str)
            .spawn()
            .map_err(|e| format!("启动下载进程失败: {}", e))?;

        Ok(DownloadResult {
            success: true,
            message: format!("下载进程已启动 (PID: {:?})", child.id()),
        })
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("当前仅支持 Windows 系统".to_string())
    }
}

/// 下载结果
#[derive(serde::Serialize)]
pub struct DownloadResult {
    success: bool,
    message: String,
}

/// 选择文件夹
#[tauri::command]
async fn select_folder(app: AppHandle, title: String) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    
    let folder_path = app
        .dialog()
        .file()
        .set_title(&title)
        .blocking_pick_folder();
    
    Ok(folder_path.map(|p| p.to_string()))
}

/// 应用补丁 - 解压7z文件并复制到游戏目录，并备份原有文件
#[tauri::command]
async fn apply_patch(
    app: AppHandle,
    patch_source_path: String,
    game_path: String,
) -> Result<ApplyPatchResult, String> {
    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;
    
    // 处理路径：如果 patch_source_path 已经包含 "resources/" 或 "Resources/" 前缀，则移除它
    let patch_lower = patch_source_path.to_lowercase();
    let patch_relative_path = if patch_lower.starts_with("resources/") {
        &patch_source_path[10..]  // 移除 "resources/" 前缀
    } else if patch_lower.starts_with("resources\\") {
        &patch_source_path[11..]  // 移除 "resources\" 前缀
    } else {
        &patch_source_path
    };
    
    // 自动添加 .7z 后缀（如果路径中没有）
    let patch_file_name = if patch_relative_path.ends_with(".7z") {
        patch_relative_path.to_string()
    } else {
        format!("{}.7z", patch_relative_path)
    };
    
    // 构建完整的补丁文件路径（相对于资源目录）
    let patch_file_path = resource_dir.join(&patch_file_name);
    let target_path = Path::new(&game_path);

    if !patch_file_path.exists() {
        return Err(format!("补丁源路径不存在: {}", patch_file_path.display()));
    }

    if !target_path.exists() {
        return Err(format!("游戏目标路径不存在: {}", game_path));
    }

    // 创建临时解压目录
    let temp_dir = std::env::temp_dir().join(format!("steam_tool_patch_{}", std::process::id()));
    let temp_path = temp_dir.clone();

    // 清理并创建临时目录
    if temp_path.exists() {
        let _ = tokio::fs::remove_dir_all(&temp_path).await;
    }
    tokio::fs::create_dir_all(&temp_path)
        .await
        .map_err(|e| format!("创建临时目录失败: {}", e))?;

    // 解压7z文件
    match extract_7z(&patch_file_path, &temp_path).await {
        Ok(_) => {}
        Err(e) => {
            // 清理临时目录
            let _ = tokio::fs::remove_dir_all(&temp_path).await;
            return Err(format!("解压补丁失败: {}", e));
        }
    }

    let mut backed_up_files: Vec<String> = Vec::new();
    let mut copied_files: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    // 递归复制解压后的文件
    match copy_dir_with_backup(&temp_path, target_path, &mut backed_up_files, &mut copied_files, &mut errors).await {
        Ok(_) => {}
        Err(e) => {
            errors.push(e);
        }
    }

    // 清理临时目录
    let _ = tokio::fs::remove_dir_all(&temp_path).await;

    Ok(ApplyPatchResult {
        success: errors.is_empty(),
        backed_up_files,
        copied_files,
        errors,
    })
}

/// 使用 sevenz-rust 解压7z文件（带超时机制）
async fn extract_7z(
    archive_path: &Path,
    output_dir: &Path,
) -> Result<(), String> {
    use tokio::time::{timeout, Duration};
    
    // 设置5分钟超时
    let result = timeout(
        Duration::from_secs(300),
        async {
            // 使用 sevenz-rust 的 decompress_file 函数解压
            sevenz_rust::decompress_file(
                archive_path.to_str().unwrap(),
                output_dir.to_str().unwrap()
            ).map_err(|e| format!("解压7z文件失败: {:?}", e))
        }
    ).await;
    
    match result {
        Ok(inner_result) => inner_result,
        Err(_) => Err("解压操作超时（超过5分钟）".to_string()),
    }
}

/// 补丁应用结果
#[derive(serde::Serialize)]
pub struct ApplyPatchResult {
    success: bool,
    backed_up_files: Vec<String>,
    copied_files: Vec<String>,
    errors: Vec<String>,
}

/// 递归复制目录，并备份已存在的文件
async fn copy_dir_with_backup(
    src: &Path,
    dst: &Path,
    backed_up_files: &mut Vec<String>,
    copied_files: &mut Vec<String>,
    errors: &mut Vec<String>,
) -> Result<(), String> {
    use tokio::fs;
    
    // 读取源目录内容
    let mut entries = fs::read_dir(src).await
        .map_err(|e| format!("无法读取源目录: {}", e))?;
    
    while let Some(entry) = entries.next_entry().await
        .map_err(|e| format!("读取目录条目失败: {}", e))? {
        
        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);
        
        if src_path.is_dir() {
            // 如果是目录，递归处理
            if !dst_path.exists() {
                fs::create_dir_all(&dst_path).await
                    .map_err(|e| format!("创建目录失败: {}", e))?;
            }
            
            Box::pin(copy_dir_with_backup(
                &src_path,
                &dst_path,
                backed_up_files,
                copied_files,
                errors,
            )).await?;
        } else {
            // 如果是文件
            if dst_path.exists() {
                // 目标文件已存在，需要备份
                match backup_file(&dst_path).await {
                    Ok(backup_path) => {
                        backed_up_files.push(backup_path.to_string_lossy().to_string());
                    }
                    Err(e) => {
                        errors.push(format!("备份文件失败 {}: {}", dst_path.display(), e));
                        continue;
                    }
                }
            }
            
            // 复制文件
            match fs::copy(&src_path, &dst_path).await {
                Ok(_) => {
                    copied_files.push(dst_path.to_string_lossy().to_string());
                }
                Err(e) => {
                    errors.push(format!("复制文件失败 {} -> {}: {}", 
                        src_path.display(), dst_path.display(), e));
                }
            }
        }
    }
    
    Ok(())
}

/// 备份文件 - 如果存在.bak则使用.bak1, .bak2等
async fn backup_file(file_path: &Path) -> Result<PathBuf, String> {
    use tokio::fs;
    
    let file_name = file_path.file_stem()
        .ok_or("无法获取文件名")?
        .to_string_lossy();
    let extension = file_path.extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let parent = file_path.parent()
        .ok_or("无法获取父目录")?;
    
    // 寻找可用的备份文件名
    let mut backup_number = 0;
    let backup_path = loop {
        let suffix = if backup_number == 0 {
            ".bak".to_string()
        } else {
            format!(".bak{}", backup_number)
        };
        
        let backup_name = format!("{}{}{}", file_name, suffix, extension);
        let backup_path = parent.join(&backup_name);
        
        if !backup_path.exists() {
            break backup_path;
        }
        
        backup_number += 1;
        
        // 防止无限循环
        if backup_number > 100 {
            return Err("无法找到可用的备份文件名".to_string());
        }
    };
    
    // 重命名原文件为备份文件
    fs::rename(file_path, &backup_path).await
        .map_err(|e| format!("重命名文件失败: {}", e))?;
    
    Ok(backup_path)
}

/// 选择 exe 文件
#[tauri::command]
async fn select_exe_file(app: AppHandle, title: String) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let file_path = app
        .dialog()
        .file()
        .set_title(&title)
        .add_filter("可执行文件", &["exe"])
        .blocking_pick_file();

    Ok(file_path.map(|p| p.to_string()))
}

/// 使用 Steamless 脱壳游戏主程序
#[tauri::command]
async fn unpack_game_exe(
    app: AppHandle,
    game_exe_path: String,
) -> Result<UnpackResult, String> {
    use std::process::Command;

    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;

    // 构建 Steamless.CLI.exe 的完整路径
    let steamless_path = resource_dir.join("steamless").join("Steamless.CLI.exe");

    if !steamless_path.exists() {
        return Err(format!("Steamless.CLI.exe 不存在: {}", steamless_path.display()));
    }

    let target_exe = Path::new(&game_exe_path);

    if !target_exe.exists() {
        return Err(format!("游戏主程序不存在: {}", game_exe_path));
    }

    if !target_exe.is_file() {
        return Err(format!("路径不是文件: {}", game_exe_path));
    }

    // 检查文件扩展名
    if let Some(ext) = target_exe.extension() {
        if ext.to_string_lossy().to_lowercase() != "exe" {
            return Err(format!("选择的文件不是 .exe 文件: {}", game_exe_path));
        }
    } else {
        return Err("选择的文件没有扩展名".to_string());
    }

    let target_exe_str = target_exe.to_string_lossy();

    // 运行 Steamless 进行脱壳
    let output = Command::new(&steamless_path)
        .arg("--quiet")
        .arg(&*target_exe_str)
        .output()
        .map_err(|e| format!("运行 Steamless 失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        // 脱壳成功，Steamless 会生成 .unpacked.exe 文件
        // Steamless 生成的文件名格式: xxx.exe.unpacked.exe
        let unpacked_exe = target_exe.with_extension("exe.unpacked.exe");
        let unpacked_path = if unpacked_exe.exists() {
            // 备份原文件为 xxx.exe.bak
            let backup_path = target_exe.with_extension("exe.bak");
            std::fs::rename(target_exe, &backup_path)
                .map_err(|e| format!("备份原文件失败: {}", e))?;
            // 将脱壳后的文件重命名为原文件名 xxx.exe
            std::fs::rename(&unpacked_exe, target_exe)
                .map_err(|e| format!("重命名脱壳文件失败: {}", e))?;
            Some(game_exe_path.clone())
        } else {
            None
        };

        Ok(UnpackResult {
            success: true,
            message: "脱壳成功".to_string(),
            unpacked_path,
        })
    } else {
        Ok(UnpackResult {
            success: false,
            message: format!("脱壳失败: {} {}", stdout, stderr),
            unpacked_path: None,
        })
    }
}

/// 脱壳结果
#[derive(serde::Serialize)]
pub struct UnpackResult {
    success: bool,
    message: String,
    unpacked_path: Option<String>,
}

/// 重命名 .EXAMPLE 文件和文件夹
async fn rename_example_files(dir: &Path) -> Result<(), String> {
    use tokio::fs;

    // 首先收集所有需要重命名的条目（避免在遍历过程中修改）
    let mut entries_to_rename: Vec<(PathBuf, PathBuf)> = Vec::new();
    let mut sub_dirs: Vec<PathBuf> = Vec::new();

    let mut entries = fs::read_dir(dir)
        .await
        .map_err(|e| format!("读取目录失败: {}", e))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| format!("读取条目失败: {}", e))? {
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if file_name_str.ends_with(".EXAMPLE") {
            // 收集需要重命名的条目（文件或文件夹）
            let new_name = &file_name_str[..file_name_str.len() - 8];
            let new_path = dir.join(new_name);
            entries_to_rename.push((path.clone(), new_path));
        } else if path.is_dir() {
            // 收集子目录以便后续递归处理
            sub_dirs.push(path);
        }
    }

    // 先递归处理子目录（在重命名父文件夹之前）
    for sub_dir in sub_dirs {
        Box::pin(rename_example_files(&sub_dir)).await?;
    }

    // 然后重命名当前目录中的 .EXAMPLE 条目
    for (old_path, new_path) in entries_to_rename {
        fs::rename(&old_path, &new_path)
            .await
            .map_err(|e| format!("重命名失败: {} -> {}: {}", old_path.display(), new_path.display(), e))?;
    }

    Ok(())
}

/// 应用 Steam 补丁基础配置
#[tauri::command]
async fn apply_steam_patch_basic(
    app: AppHandle,
    game_path: String,
    _game_id: String,
    steam_app_id: String,
    use_experimental: bool,
    emulator_mode: Option<i32>, // 0=标准模式(steam_api.dll), 1=高级模式(steamclient.dll)
) -> Result<BasicConfigResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;
    use std::process::Command;

    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;

    let game_dir = Path::new(&game_path);

    if !game_dir.exists() {
        return Err(format!("游戏路径不存在: {}", game_path));
    }

    let steam_settings_dir = game_dir.join("steam_settings");
    let mode = emulator_mode.unwrap_or(0);

    // ========== 第1步: 复制 steam_settings.EXAMPLE 文件夹 ==========
    let example_dir = resource_dir.join("gbe_fork/steam_settings.EXAMPLE");
    
    if example_dir.exists() {
        // 如果已存在 steam_settings，先删除
        if steam_settings_dir.exists() {
            fs::remove_dir_all(&steam_settings_dir)
                .await
                .map_err(|e| format!("删除旧 steam_settings 失败: {}", e))?;
        }

        // 递归复制示例文件夹
        copy_dir_recursive(&example_dir, &steam_settings_dir)
            .await
            .map_err(|e| format!("复制 steam_settings.EXAMPLE 失败: {}", e))?;

        // 重命名 .EXAMPLE 文件
        rename_example_files(&steam_settings_dir)
            .await
            .map_err(|e| format!("重命名示例文件失败: {}", e))?;
    }

    // ========== 第2步: 判断游戏架构 ==========
    let is_64bit = game_dir.read_dir()
        .map_err(|e| format!("读取游戏目录失败: {}", e))?
        .filter_map(|e| e.ok())
        .any(|e| {
            if let Some(name) = e.file_name().to_str() {
                name.contains("64") || name.contains("x64")
            } else {
                false
            }
        });

    // ========== 第3步: 根据模式处理 DLL ==========
    if mode == 0 {
        // ========== 标准模式: 替换 steam_api.dll ==========
        let api_dll_name = if is_64bit { "steam_api64.dll" } else { "steam_api.dll" };
        let original_api_path = game_dir.join(&api_dll_name);

        // 检查是否存在 steam_api.dll
        if !original_api_path.exists() {
            return Err(format!(
                "未找到 {}！\n\n可能原因：\n1. 选择的游戏目录不正确\n2. 游戏使用 steamclient.dll 而非 steam_api.dll\n3. 请先尝试「高级模式」",
                api_dll_name
            ));
        }

        // 3.1 备份原 DLL
        let backup_path = game_dir.join(format!("{}.bak", api_dll_name));
        if !backup_path.exists() {
            fs::copy(&original_api_path, &backup_path)
                .await
                .map_err(|e| format!("备份原 DLL 失败: {}", e))?;
        }

        // 3.2 生成 steam_interfaces.txt
        let tool_name = if is_64bit {
            "tools/generate_interfaces/generate_interfaces_x64.exe"
        } else {
            "tools/generate_interfaces/generate_interfaces_x32.exe"
        };
        let tool_path = resource_dir.join("gbe_fork").join(tool_name);

        if tool_path.exists() {
            let tool_path_clone = tool_path.clone();
            let original_api_path_clone = original_api_path.clone();
            let game_dir_clone = game_dir.to_path_buf();
            let steam_settings_dir_clone = steam_settings_dir.clone();
            
            let result = tokio::task::spawn_blocking(move || {
                Command::new(&tool_path_clone)
                    .arg(&original_api_path_clone)
                    .current_dir(&game_dir_clone)
                    .output()
            }).await.map_err(|e| format!("运行 generate_interfaces 失败: {}", e))?;

            if let Ok(output) = result {
                if output.status.success() {
                    // 移动生成的 steam_interfaces.txt 到 steam_settings
                    let generated_txt = game_dir.join("steam_interfaces.txt");
                    if generated_txt.exists() {
                        let target_txt = steam_settings_dir_clone.join("steam_interfaces.txt");
                        fs::rename(&generated_txt, &target_txt)
                            .await
                            .map_err(|e| format!("移动 steam_interfaces.txt 失败: {}", e))?;
                    }
                }
            }
        }

        // 3.3 替换 steam_api.dll
        let source_dll = if is_64bit {
            if use_experimental {
                "gbe_fork/experimental/x64/steam_api64.dll"
            } else {
                "gbe_fork/regular/x64/steam_api64.dll"
            }
        } else {
            if use_experimental {
                "gbe_fork/experimental/x32/steam_api.dll"
            } else {
                "gbe_fork/regular/x32/steam_api.dll"
            }
        };

        let source_path = resource_dir.join(source_dll);
        if source_path.exists() {
            fs::copy(&source_path, &original_api_path)
                .await
                .map_err(|e| format!("复制 DLL 失败: {}", e))?;
        } else {
            return Err(format!("源 DLL 文件不存在: {}", source_path.display()));
        }
    } else {
        // ========== 高级模式: 替换 steamclient.dll + steam_api.dll ==========
        
        // 4.1 替换 steamclient.dll
        let client_dll_name = if is_64bit { "steamclient64.dll" } else { "steamclient.dll" };
        let original_client_path = game_dir.join(&client_dll_name);

        // 检查是否存在 steamclient.dll
        if !original_client_path.exists() {
            return Err(format!(
                "未找到 {}！\n\n可能原因：\n1. 选择的游戏目录不正确\n2. 该游戏不支持高级模式\n3. 请尝试「标准模式」",
                client_dll_name
            ));
        }

        // 备份原 steamclient.dll
        let backup_path = game_dir.join(format!("{}.bak", client_dll_name));
        if !backup_path.exists() {
            fs::copy(&original_client_path, &backup_path)
                .await
                .map_err(|e| format!("备份 steamclient 失败: {}", e))?;
        }

        // 复制实验版 steamclient.dll
        let client_source = if is_64bit {
            "gbe_fork/steamclient_experimental/steamclient64.dll"
        } else {
            "gbe_fork/steamclient_experimental/steamclient.dll"
        };

        let client_source_path = resource_dir.join(client_source);
        if client_source_path.exists() {
            fs::copy(&client_source_path, &original_client_path)
                .await
                .map_err(|e| format!("复制 steamclient 失败: {}", e))?;
        } else {
            return Err(format!("源 steamclient 文件不存在: {}", client_source_path.display()));
        }

        // 4.2 同步替换 steam_api.dll（实验版）
        let api_dll_name = if is_64bit { "steam_api64.dll" } else { "steam_api.dll" };
        let api_source = if is_64bit {
            "gbe_fork/experimental/x64/steam_api64.dll"
        } else {
            "gbe_fork/experimental/x32/steam_api.dll"
        };

        let api_source_path = resource_dir.join(api_source);
        let api_target_path = game_dir.join(&api_dll_name);

        if api_source_path.exists() {
            // 备份原 steam_api.dll（如果存在）
            if api_target_path.exists() {
                let api_backup_path = game_dir.join(format!("{}.bak", api_dll_name));
                if !api_backup_path.exists() {
                    fs::copy(&api_target_path, &api_backup_path)
                        .await
                        .map_err(|e| format!("备份 steam_api 失败: {}", e))?;
                }
            }

            // 复制实验版 steam_api.dll
            fs::copy(&api_source_path, &api_target_path)
                .await
                .map_err(|e| format!("复制 steam_api 失败: {}", e))?;
        } else {
            return Err(format!("源 steam_api 文件不存在: {}", api_source_path.display()));
        }
    }

    // ========== 第4步: 双路径写入 steam_appid.txt ==========
    // 4.1 steam_settings/steam_appid.txt
    let appid_path_settings = steam_settings_dir.join("steam_appid.txt");
    let mut appid_file_settings = fs::File::create(&appid_path_settings)
        .await
        .map_err(|e| format!("创建 steam_settings/steam_appid.txt 失败: {}", e))?;
    appid_file_settings.write_all(steam_app_id.as_bytes())
        .await
        .map_err(|e| format!("写入 steam_settings/steam_appid.txt 失败: {}", e))?;

    // 4.2 游戏根目录/steam_appid.txt
    let appid_path_root = game_dir.join("steam_appid.txt");
    let mut appid_file_root = fs::File::create(&appid_path_root)
        .await
        .map_err(|e| format!("创建根目录 steam_appid.txt 失败: {}", e))?;
    appid_file_root.write_all(steam_app_id.as_bytes())
        .await
        .map_err(|e| format!("写入根目录 steam_appid.txt 失败: {}", e))?;

    // ========== 第5步: 写入基础配置文件 ==========
    // 5.1 写入 configs.main.ini - 核心配置（强制局域网模式）
    let main_config_path = steam_settings_dir.join("configs.main.ini");
    let main_config_content = r#"[main]
# 强制局域网模式 - 屏蔽外网校验，实现免Steam联机
force_lan_only = 1

# 启用局域网广播
enable_lan_broadcast = 1

# 匹配服务器列表类型
# 0=始终返回局域网服务器列表
matchmaking_server_list_actual_type = 0

# 禁用Steam网络检查
disable_steam_network_check = 1

# 低延迟联机模式
lan_broadcast_interval = 300

# 允许未知统计
allow_unknown_stats = 1

# 最大联机人数
max_lobby_players = 32
"#;
    
    let mut main_file = fs::File::create(&main_config_path)
        .await
        .map_err(|e| format!("创建 configs.main.ini 失败: {}", e))?;
    main_file.write_all(main_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.main.ini 失败: {}", e))?;

    // 5.2 写入 configs.user.ini - 用户配置
    let user_config_path = steam_settings_dir.join("configs.user.ini");
    let user_config_content = r#"[user]
# 默认存档路径：%appdata%\GSE Saves
"#;
    
    let mut user_file = fs::File::create(&user_config_path)
        .await
        .map_err(|e| format!("创建 configs.user.ini 失败: {}", e))?;
    user_file.write_all(user_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.user.ini 失败: {}", e))?;

    // 5.3 写入 configs.app.ini - 应用配置（DLC解锁）
    let app_config_path = steam_settings_dir.join("configs.app.ini");
    let app_config_content = r#"
# default=public
branch_name=public

[app::paths]

[app::dlcs]
# 一键解锁全部DLC
unlock_all=1
"#;
    
    let mut app_file = fs::File::create(&app_config_path)
        .await
        .map_err(|e| format!("创建 configs.app.ini 失败: {}", e))?;
    app_file.write_all(app_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.app.ini 失败: {}", e))?;

    // 5.4 写入 custom_broadcasts.txt - 局域网广播配置
    let broadcasts_path = steam_settings_dir.join("custom_broadcasts.txt");
    let broadcasts_content = r#"192.168.1.0/24
192.168.0.0/24
10.0.0.0/24
"#;
    
    let mut broadcasts_file = fs::File::create(&broadcasts_path)
        .await
        .map_err(|e| format!("创建 custom_broadcasts.txt 失败: {}", e))?;
    broadcasts_file.write_all(broadcasts_content.as_bytes())
        .await
        .map_err(|e| format!("写入 custom_broadcasts.txt 失败: {}", e))?;

    Ok(BasicConfigResult {
        success: true,
        message: "基础配置已应用".to_string(),
    })
}

/// 基础配置结果
#[derive(serde::Serialize)]
pub struct BasicConfigResult {
    success: bool,
    message: String,
}

/// 保存局域网联机配置
#[tauri::command]
async fn save_lan_multiplayer_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 文件夹失败: {}", e))?;

    // 保存 custom_broadcasts.txt
    if let Some(broadcasts) = config.get("customBroadcasts").and_then(|v| v.as_array()) {
        let broadcasts_path = steam_settings_dir.join("custom_broadcasts.txt");
        let content: Vec<String> = broadcasts
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .filter(|s| !s.is_empty())
            .collect();

        if !content.is_empty() {
            let mut file = fs::File::create(&broadcasts_path)
                .await
                .map_err(|e| format!("创建 custom_broadcasts.txt 失败: {}", e))?;
            file.write_all(content.join("\n").as_bytes())
                .await
                .map_err(|e| format!("写入 custom_broadcasts.txt 失败: {}", e))?;
        }
    }

    // 保存 auto_accept_invite.txt
    if let Some(auto_accept) = config.get("autoAcceptInvite").and_then(|v| v.as_str()) {
        if auto_accept != "none" {
            let invite_path = steam_settings_dir.join("auto_accept_invite.txt");
            let content = if auto_accept == "all" {
                "".to_string()
            } else if let Some(whitelist) = config.get("whitelist").and_then(|v| v.as_array()) {
                whitelist
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                "".to_string()
            };

            let mut file = fs::File::create(&invite_path)
                .await
                .map_err(|e| format!("创建 auto_accept_invite.txt 失败: {}", e))?;
            file.write_all(content.as_bytes())
                .await
                .map_err(|e| format!("写入 auto_accept_invite.txt 失败: {}", e))?;
        }
    }

    Ok(ConfigSaveResult {
        success: true,
        message: "局域网联机配置已保存".to_string(),
    })
}

/// 配置保存结果
#[derive(serde::Serialize)]
pub struct ConfigSaveResult {
    success: bool,
    message: String,
}

/// 保存成就配置
#[tauri::command]
async fn save_achievements_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 文件夹失败: {}", e))?;

    // 保存 achievements.json
    if let Some(achievements) = config.get("achievements").and_then(|v| v.as_array()) {
        let achievements_path = steam_settings_dir.join("achievements.json");
        let achievements_data: Vec<serde_json::Value> = achievements.clone();

        let json_content = serde_json::to_string_pretty(&achievements_data)
            .map_err(|e| format!("序列化成就数据失败: {}", e))?;

        let mut file = fs::File::create(&achievements_path)
            .await
            .map_err(|e| format!("创建 achievements.json 失败: {}", e))?;
        file.write_all(json_content.as_bytes())
            .await
            .map_err(|e| format!("写入 achievements.json 失败: {}", e))?;
    }

    Ok(ConfigSaveResult {
        success: true,
        message: "成就配置已保存".to_string(),
    })
}

/// 保存物品配置
#[tauri::command]
async fn save_items_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 文件夹失败: {}", e))?;

    // 保存 items.json
    if let Some(items) = config.get("items").and_then(|v| v.as_array()) {
        let items_path = steam_settings_dir.join("items.json");
        let items_data: Vec<serde_json::Value> = items.clone();

        let json_content = serde_json::to_string_pretty(&items_data)
            .map_err(|e| format!("序列化物品数据失败: {}", e))?;

        let mut file = fs::File::create(&items_path)
            .await
            .map_err(|e| format!("创建 items.json 失败: {}", e))?;
        file.write_all(json_content.as_bytes())
            .await
            .map_err(|e| format!("写入 items.json 失败: {}", e))?;
    }

    Ok(ConfigSaveResult {
        success: true,
        message: "物品配置已保存".to_string(),
    })
}

/// 保存用户配置
#[tauri::command]
async fn save_user_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 文件夹失败: {}", e))?;

    // 保存 configs.user.ini
    let config_path = steam_settings_dir.join("configs.user.ini");
    let mut content = String::new();

    if let Some(username) = config.get("username").and_then(|v| v.as_str()) {
        content.push_str(&format!("user_name={}\n", username));
    }

    if let Some(language) = config.get("language").and_then(|v| v.as_str()) {
        content.push_str(&format!("language={}\n", language));
    }

    if let Some(steam_id) = config.get("steamId").and_then(|v| v.as_str()) {
        if !steam_id.is_empty() {
            content.push_str(&format!("account_id={}\n", steam_id));
        }
    }

    if let Some(saves_folder) = config.get("savesFolderName").and_then(|v| v.as_str()) {
        if !saves_folder.is_empty() {
            content.push_str(&format!("saves_folder_name={}\n", saves_folder));
        }
    }

    if let Some(local_save) = config.get("localSavePath").and_then(|v| v.as_str()) {
        if !local_save.is_empty() {
            content.push_str(&format!("local_save_path={}\n", local_save));
        }
    }

    let mut file = fs::File::create(&config_path)
        .await
        .map_err(|e| format!("创建 configs.user.ini 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.user.ini 失败: {}", e))?;

    // 复制头像文件
    if let Some(avatar_path) = config.get("avatarPath").and_then(|v| v.as_str()) {
        if !avatar_path.is_empty() {
            let avatar_source = Path::new(avatar_path);
            if avatar_source.exists() {
                let ext = avatar_source.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("jpg");
                let avatar_target = steam_settings_dir.join(format!("account_avatar.{}", ext));
                fs::copy(avatar_source, avatar_target)
                    .await
                    .map_err(|e| format!("复制头像文件失败: {}", e))?;
            }
        }
    }

    Ok(ConfigSaveResult {
        success: true,
        message: "用户配置已保存".to_string(),
    })
}

/// 保存主配置
#[tauri::command]
async fn save_main_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 文件夹失败: {}", e))?;

    // 保存 configs.main.ini
    let config_path = steam_settings_dir.join("configs.main.ini");
    let mut content = String::new();

    if let Some(new_ticket) = config.get("newAppTicket").and_then(|v| v.as_bool()) {
        content.push_str(&format!("new_app_ticket={}\n", if new_ticket { "1" } else { "0" }));
    }

    if let Some(gc_token) = config.get("gcToken").and_then(|v| v.as_bool()) {
        content.push_str(&format!("gc_token={}\n", if gc_token { "1" } else { "0" }));
    }

    if let Some(offline) = config.get("offlineMode").and_then(|v| v.as_bool()) {
        content.push_str(&format!("offline={}\n", if offline { "1" } else { "0" }));
    }

    if let Some(disable_network) = config.get("disableNetworking").and_then(|v| v.as_bool()) {
        content.push_str(&format!("disable_networking={}\n", if disable_network { "1" } else { "0" }));
    }

    if let Some(ticket) = config.get("encryptedAppTicket").and_then(|v| v.as_str()) {
        if !ticket.is_empty() {
            content.push_str(&format!("ticket={}\n", ticket));
        }
    }

    let mut file = fs::File::create(&config_path)
        .await
        .map_err(|e| format!("创建 configs.main.ini 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.main.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "主配置已保存".to_string(),
    })
}

/// 保存排行榜配置
#[tauri::command]
async fn save_leaderboards_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 文件夹失败: {}", e))?;

    // 保存 leaderboards.txt
    if let Some(leaderboards) = config.get("leaderboards").and_then(|v| v.as_array()) {
        let leaderboards_path = steam_settings_dir.join("leaderboards.txt");
        let mut content = String::new();

        for lb in leaderboards {
            if let (Some(name), Some(sort), Some(display)) = (
                lb.get("name").and_then(|v| v.as_str()),
                lb.get("sortMethod").and_then(|v| v.as_i64()),
                lb.get("displayType").and_then(|v| v.as_i64()),
            ) {
                content.push_str(&format!("{}={}={}\n", name, sort, display));
            }
        }

        let mut file = fs::File::create(&leaderboards_path)
            .await
            .map_err(|e| format!("创建 leaderboards.txt 失败: {}", e))?;
        file.write_all(content.as_bytes())
            .await
            .map_err(|e| format!("写入 leaderboards.txt 失败: {}", e))?;
    }

    Ok(ConfigSaveResult {
        success: true,
        message: "排行榜配置已保存".to_string(),
    })
}

/// 保存统计配置
#[tauri::command]
async fn save_stats_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 文件夹失败: {}", e))?;

    // 保存 stats.json
    if let Some(stats) = config.get("stats").and_then(|v| v.as_array()) {
        let stats_path = steam_settings_dir.join("stats.json");
        let stats_data: Vec<serde_json::Value> = stats.clone();

        let json_content = serde_json::to_string_pretty(&stats_data)
            .map_err(|e| format!("序列化统计数据失败: {}", e))?;

        let mut file = fs::File::create(&stats_path)
            .await
            .map_err(|e| format!("创建 stats.json 失败: {}", e))?;
        file.write_all(json_content.as_bytes())
            .await
            .map_err(|e| format!("写入 stats.json 失败: {}", e))?;
    }

    // 保存 allow_unknown_stats 到 configs.main.ini
    if let Some(allow_unknown) = config.get("allowUnknownStats").and_then(|v| v.as_bool()) {
        let config_path = steam_settings_dir.join("configs.main.ini");
        let content = format!("allow_unknown_stats={}\n", if allow_unknown { "1" } else { "0" });

        // 追加到文件
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config_path)
            .await
            .map_err(|e| format!("打开 configs.main.ini 失败: {}", e))?;
        file.write_all(content.as_bytes())
            .await
            .map_err(|e| format!("写入 configs.main.ini 失败: {}", e))?;
    }

    Ok(ConfigSaveResult {
        success: true,
        message: "统计配置已保存".to_string(),
    })
}

/// 保存 DLC 配置
#[tauri::command]
async fn save_dlc_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 文件夹失败: {}", e))?;

    let config_path = steam_settings_dir.join("configs.app.ini");
    
    // 读取现有配置（如果存在）
    let mut existing_content = String::new();
    let mut before_dlcs_section = String::new();
    let mut after_dlcs_section = String::new();
    
    if config_path.exists() {
        existing_content = fs::read_to_string(&config_path)
            .await
            .unwrap_or_default();
        
        // 解析现有配置，分离出 [app::dlcs] 部分之前和之后的内容
        let mut in_dlcs_section = false;
        let mut found_dlcs_section = false;
        for line in existing_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("[app::dlcs]") {
                found_dlcs_section = true;
                in_dlcs_section = true;
            } else if in_dlcs_section && trimmed.starts_with('[') && trimmed.ends_with(']') {
                // 进入了新的section
                in_dlcs_section = false;
                after_dlcs_section.push_str(line);
                after_dlcs_section.push('\n');
            } else if in_dlcs_section {
                // 在dlcs section中，跳过
            } else if found_dlcs_section {
                after_dlcs_section.push_str(line);
                after_dlcs_section.push('\n');
            } else {
                before_dlcs_section.push_str(line);
                before_dlcs_section.push('\n');
            }
        }
    }

    // 构建新的 DLC 配置内容
    let mut dlcs_content = String::new();
    dlcs_content.push_str("[app::dlcs]\n");
    
    // 检查是否启用 unlock_all
    let unlock_all = config.get("unlockAll").and_then(|v| v.as_bool()).unwrap_or(true);
    dlcs_content.push_str(&format!("unlock_all={}\n", if unlock_all { "1" } else { "0" }));
    
    // 添加单个DLC配置（仅在unlock_all=0时有效，但也保存下来）
    if let Some(dlcs) = config.get("dlcs").and_then(|v| v.as_array()) {
        for dlc in dlcs {
            if let (Some(app_id), Some(name)) = (
                dlc.get("appId").and_then(|v| v.as_i64()),
                dlc.get("name").and_then(|v| v.as_str()),
            ) {
                if !name.is_empty() {
                    dlcs_content.push_str(&format!("{}={}\n", app_id, name));
                }
            }
        }
    }
    dlcs_content.push('\n');

    // 合并配置
    let mut final_content = String::new();
    final_content.push_str(&before_dlcs_section);
    final_content.push_str(&dlcs_content);
    final_content.push_str(&after_dlcs_section);

    // 如果之前没有configs.app.ini，创建默认内容
    if existing_content.is_empty() {
        final_content = format!(r#"# default=public
branch_name=public

[app::paths]

{}
"#, dlcs_content);
    }

    // 写入配置
    let mut file = fs::File::create(&config_path)
        .await
        .map_err(|e| format!("创建 configs.app.ini 失败: {}", e))?;
    file.write_all(final_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.app.ini 失败: {}", e))?;

    // 保存 depots.txt
    if let Some(depots) = config.get("depots").and_then(|v| v.as_array()) {
        let depots_path = steam_settings_dir.join("depots.txt");
        let mut content = String::new();

        for depot in depots {
            if let Some(id) = depot.get("id").and_then(|v| v.as_i64()) {
                if id > 0 {
                    content.push_str(&format!("{}\n", id));
                }
            }
        }

        let mut file = fs::File::create(&depots_path)
            .await
            .map_err(|e| format!("创建 depots.txt 失败: {}", e))?;
        file.write_all(content.as_bytes())
            .await
            .map_err(|e| format!("写入 depots.txt 失败: {}", e))?;
    }

    // 保存 installed_app_ids.txt
    if let Some(apps) = config.get("installedApps").and_then(|v| v.as_array()) {
        let apps_path = steam_settings_dir.join("installed_app_ids.txt");
        let mut content = String::new();

        for app in apps {
            if let Some(id) = app.get("id").and_then(|v| v.as_i64()) {
                if id > 0 {
                    content.push_str(&format!("{}\n", id));
                }
            }
        }

        let mut file = fs::File::create(&apps_path)
            .await
            .map_err(|e| format!("创建 installed_app_ids.txt 失败: {}", e))?;
        file.write_all(content.as_bytes())
            .await
            .map_err(|e| format!("写入 installed_app_ids.txt 失败: {}", e))?;
    }

    // 保存 DLC 文件夹路径到 configs.app.ini 的 [app::paths] 段
    if let Some(folders) = config.get("dlcFolders").and_then(|v| v.as_array()) {
        let mut paths_content = String::new();
        
        for folder in folders {
            if let (Some(path), Some(dlcs)) = (
                folder.get("path").and_then(|v| v.as_str()),
                folder.get("dlcs").and_then(|v| v.as_array()),
            ) {
                // 计算相对于游戏目录的路径
                let folder_path = Path::new(path);
                let game_path_obj = Path::new(&game_path);
                
                // 尝试获取相对路径
                let relative_path = folder_path.strip_prefix(game_path_obj)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| path.to_string());
                
                // 为每个DLC添加路径映射
                for dlc in dlcs {
                    if let Some(app_id) = dlc.get("appId").and_then(|v| v.as_i64()) {
                        // 使用相对路径格式
                        let path_for_ini = if relative_path.starts_with("..") || relative_path.starts_with("./") {
                            relative_path.clone()
                        } else if relative_path.is_empty() {
                            ".".to_string()
                        } else {
                            format!("./{}", relative_path)
                        };
                        paths_content.push_str(&format!("{}={}\n", app_id, path_for_ini));
                    }
                }
            }
        }
        
        // 如果存在DLC路径，需要更新configs.app.ini
        if !paths_content.is_empty() {
            let config_path = steam_settings_dir.join("configs.app.ini");
            let existing = fs::read_to_string(&config_path).await.unwrap_or_default();
            
            // 检查是否已有[app::paths]段
            let mut new_content = String::new();
            let mut in_paths_section = false;
            let mut paths_section_added = false;
            
            for line in existing.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("[app::paths]") {
                    in_paths_section = true;
                    if !paths_section_added {
                        new_content.push_str("[app::paths]\n");
                        new_content.push_str(&paths_content);
                        paths_section_added = true;
                    }
                } else if in_paths_section && trimmed.starts_with('[') && trimmed.ends_with(']') {
                    in_paths_section = false;
                    new_content.push_str(line);
                    new_content.push('\n');
                } else if !in_paths_section {
                    new_content.push_str(line);
                    new_content.push('\n');
                }
            }
            
            // 如果没有找到[app::paths]段，在[app::dlcs]之前添加
            if !paths_section_added {
                let mut final_content = String::new();
                let mut dlcs_section_found = false;
                for line in new_content.lines() {
                    if line.trim().starts_with("[app::dlcs]") && !dlcs_section_found {
                        dlcs_section_found = true;
                        final_content.push_str("[app::paths]\n");
                        final_content.push_str(&paths_content);
                        final_content.push('\n');
                    }
                    final_content.push_str(line);
                    final_content.push('\n');
                }
                new_content = final_content;
            }
            
            let mut file = fs::File::create(&config_path)
                .await
                .map_err(|e| format!("更新 configs.app.ini 失败: {}", e))?;
            file.write_all(new_content.as_bytes())
                .await
                .map_err(|e| format!("写入 configs.app.ini 失败: {}", e))?;
        }
    }

    Ok(ConfigSaveResult {
        success: true,
        message: "DLC 配置已保存".to_string(),
    })
}

/// 扫描DLC文件夹
#[tauri::command]
async fn scan_dlc_folder(
    app: AppHandle,
    _game_path: String,
) -> Result<ScanDlcResult, String> {
    use tauri_plugin_dialog::DialogExt;
    use tokio::fs;
    
    // 打开文件夹选择对话框
    let folder_path = app.dialog()
        .file()
        .set_title("选择DLC文件夹")
        .blocking_pick_folder();
    
    match folder_path {
        Some(path) => {
            let path_str = path.to_string();
            let path_obj = Path::new(&path_str);
            let mut dlcs = Vec::new();
            
            // 读取文件夹内容
            let mut entries = fs::read_dir(path_obj)
                .await
                .map_err(|e| format!("读取文件夹失败: {}", e))?;
            
            while let Some(entry) = entries.next_entry()
                .await
                .map_err(|e| format!("读取文件失败: {}", e))? 
            {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                
                // 检查是否是.pak文件
                if file_name_str.ends_with(".pak") {
                    // 尝试从文件名提取DLC ID
                    // 格式如: re_dlc_stm_1456360.pak
                    if let Some(id_str) = extract_dlc_id_from_filename(&file_name_str) {
                        if let Ok(app_id) = id_str.parse::<i64>() {
                            dlcs.push(serde_json::json!({
                                "appId": app_id,
                                "name": file_name_str.trim_end_matches(".pak")
                            }));
                        }
                    }
                }
            }
            
            Ok(ScanDlcResult {
                canceled: false,
                folder_path: Some(path_str),
                dlcs: Some(dlcs),
            })
        }
        None => Ok(ScanDlcResult {
            canceled: true,
            folder_path: None,
            dlcs: None,
        }),
    }
}

/// 扫描DLC结果
#[derive(serde::Serialize)]
pub struct ScanDlcResult {
    canceled: bool,
    folder_path: Option<String>,
    dlcs: Option<Vec<serde_json::Value>>,
}

/// 从文件名提取DLC ID
fn extract_dlc_id_from_filename(filename: &str) -> Option<&str> {
    // 支持的格式:
    // re_dlc_stm_1456360.pak -> 1456360
    // dlc_123456.pak -> 123456
    // 1456360.pak -> 1456360
    
    let name_without_ext = filename.trim_end_matches(".pak");
    
    // 尝试从最后一个下划线后提取数字
    if let Some(last_underscore) = name_without_ext.rfind('_') {
        let after_underscore = &name_without_ext[last_underscore + 1..];
        if after_underscore.chars().all(|c| c.is_ascii_digit()) {
            return Some(after_underscore);
        }
    }
    
    // 如果整个文件名都是数字
    if name_without_ext.chars().all(|c| c.is_ascii_digit()) {
        return Some(name_without_ext);
    }
    
    None
}

/// 保存控制器配置
#[tauri::command]
async fn save_controller_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    let controller_dir = steam_settings_dir.join("controller");
    fs::create_dir_all(&controller_dir)
        .await
        .map_err(|e| format!("创建 controller 文件夹失败: {}", e))?;

    // 保存每个 action set
    if let Some(action_sets) = config.get("actionSets").and_then(|v| v.as_array()) {
        for set in action_sets {
            if let Some(name) = set.get("name").and_then(|v| v.as_str()) {
                let set_path = controller_dir.join(format!("{}.txt", name));
                let mut content = String::new();

                // 数字动作
                if let Some(digital) = set.get("digitalActions").and_then(|v| v.as_array()) {
                    for action in digital {
                        if let (Some(action_name), Some(button)) = (
                            action.get("name").and_then(|v| v.as_str()),
                            action.get("button").and_then(|v| v.as_str()),
                        ) {
                            content.push_str(&format!("{}={}\n", action_name, button));
                        }
                    }
                }

                // 模拟动作
                if let Some(analog) = set.get("analogActions").and_then(|v| v.as_array()) {
                    for action in analog {
                        if let (Some(action_name), Some(analog_input), Some(mode)) = (
                            action.get("name").and_then(|v| v.as_str()),
                            action.get("analog").and_then(|v| v.as_str()),
                            action.get("mode").and_then(|v| v.as_str()),
                        ) {
                            content.push_str(&format!("{}={}={}\n", action_name, analog_input, mode));
                        }
                    }
                }

                let mut file = fs::File::create(&set_path)
                    .await
                    .map_err(|e| format!("创建 {}.txt 失败: {}", name, e))?;
                file.write_all(content.as_bytes())
                    .await
                    .map_err(|e| format!("写入 {}.txt 失败: {}", name, e))?;
            }
        }
    }

    // 复制图标注释文件夹
    if let Some(glyphs_folder) = config.get("glyphsFolder").and_then(|v| v.as_str()) {
        if !glyphs_folder.is_empty() {
            let glyphs_source = Path::new(glyphs_folder);
            if glyphs_source.exists() && glyphs_source.is_dir() {
                let glyphs_target = controller_dir.join("glyphs");
                copy_dir_recursive(glyphs_source, &glyphs_target)
                    .await
                    .map_err(|e| format!("复制图标注释文件夹失败: {}", e))?;
            }
        }
    }

    Ok(ConfigSaveResult {
        success: true,
        message: "控制器配置已保存".to_string(),
    })
}

/// 递归复制目录
async fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    use tokio::fs;

    fs::create_dir_all(dst)
        .await
        .map_err(|e| format!("创建目录失败: {}", e))?;

    let mut entries = fs::read_dir(src).await
        .map_err(|e| format!("读取目录失败: {}", e))?;

    while let Some(entry) = entries.next_entry().await
        .map_err(|e| format!("读取目录条目失败: {}", e))? {
        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if src_path.is_dir() {
            Box::pin(copy_dir_recursive(&src_path, &dst_path)).await?;
        } else {
            fs::copy(&src_path, &dst_path)
                .await
                .map_err(|e| format!("复制文件失败: {}", e))?;
        }
    }

    Ok(())
}

/// 保存 Overlay 配置
#[tauri::command]
async fn save_overlay_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 文件夹失败: {}", e))?;

    // 保存 configs.overlay.ini
    let config_path = steam_settings_dir.join("configs.overlay.ini");
    let mut content = String::new();

    if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
        content.push_str(&format!("enable_experimental_overlay={}\n", if enabled { "1" } else { "0" }));
    }

    if let Some(show_fps) = config.get("showFPS").and_then(|v| v.as_bool()) {
        content.push_str(&format!("show_fps={}\n", if show_fps { "1" } else { "0" }));
    }

    if let Some(show_clock) = config.get("showClock").and_then(|v| v.as_bool()) {
        content.push_str(&format!("show_clock={}\n", if show_clock { "1" } else { "0" }));
    }

    if let Some(show_notifications) = config.get("showNotifications").and_then(|v| v.as_bool()) {
        content.push_str(&format!("show_notifications={}\n", if show_notifications { "1" } else { "0" }));
    }

    let mut file = fs::File::create(&config_path)
        .await
        .map_err(|e| format!("创建 configs.overlay.ini 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.overlay.ini 失败: {}", e))?;

    // 复制声音文件
    let sounds_dir = steam_settings_dir.join("sounds");
    fs::create_dir_all(&sounds_dir)
        .await
        .map_err(|e| format!("创建 sounds 文件夹失败: {}", e))?;

    if let Some(achievement_sound) = config.get("achievementSoundPath").and_then(|v| v.as_str()) {
        if !achievement_sound.is_empty() {
            let source = Path::new(achievement_sound);
            if source.exists() {
                let target = sounds_dir.join("overlay_achievement_notification.wav");
                fs::copy(source, target)
                    .await
                    .map_err(|e| format!("复制成就声音文件失败: {}", e))?;
            }
        }
    }

    if let Some(friend_sound) = config.get("friendSoundPath").and_then(|v| v.as_str()) {
        if !friend_sound.is_empty() {
            let source = Path::new(friend_sound);
            if source.exists() {
                let target = sounds_dir.join("overlay_friend_notification.wav");
                fs::copy(source, target)
                    .await
                    .map_err(|e| format!("复制好友声音文件失败: {}", e))?;
            }
        }
    }

    // 复制字体文件
    if let Some(font_path) = config.get("fontPath").and_then(|v| v.as_str()) {
        if !font_path.is_empty() {
            let source = Path::new(font_path);
            if source.exists() {
                let ext = source.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("ttf");
                let target = steam_settings_dir.join(format!("font.{}", ext));
                fs::copy(source, target)
                    .await
                    .map_err(|e| format!("复制字体文件失败: {}", e))?;
            }
        }
    }

    Ok(ConfigSaveResult {
        success: true,
        message: "Overlay 配置已保存".to_string(),
    })
}

/// 加载局域网联机配置
#[tauri::command]
async fn load_lan_multiplayer_config(
    game_path: String,
) -> Result<ConfigLoadResult<serde_json::Value>, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");

    if !steam_settings_dir.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    // 读取 custom_broadcasts.txt
    let broadcasts_path = steam_settings_dir.join("custom_broadcasts.txt");
    let custom_broadcasts = if broadcasts_path.exists() {
        fs::read_to_string(&broadcasts_path)
            .await
            .unwrap_or_default()
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    // 读取 auto_accept_invite.txt
    let invite_path = steam_settings_dir.join("auto_accept_invite.txt");
    let (auto_accept_invite, whitelist) = if invite_path.exists() {
        let content = fs::read_to_string(&invite_path).await.unwrap_or_default();
        let whitelist: Vec<String> = content
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if whitelist.is_empty() {
            ("all".to_string(), Vec::new())
        } else {
            ("whitelist".to_string(), whitelist)
        }
    } else {
        ("none".to_string(), Vec::new())
    };

    let config = serde_json::json!({
        "enabled": custom_broadcasts.len() > 0 || auto_accept_invite != "none",
        "customBroadcasts": custom_broadcasts,
        "autoAcceptInvite": auto_accept_invite,
        "whitelist": whitelist,
        "listenPort": 47584,
    });

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 加载成就配置
#[tauri::command]
async fn load_achievements_config(
    game_path: String,
) -> Result<ConfigLoadResult<serde_json::Value>, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    let achievements_path = steam_settings_dir.join("achievements.json");

    if !achievements_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&achievements_path)
        .await
        .map_err(|e| format!("读取 achievements.json 失败: {}", e))?;

    let achievements: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析 achievements.json 失败: {}", e))?;

    let config = serde_json::json!({
        "achievements": achievements,
    });

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 加载物品配置
#[tauri::command]
async fn load_items_config(
    game_path: String,
) -> Result<ConfigLoadResult<serde_json::Value>, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    let items_path = steam_settings_dir.join("items.json");

    if !items_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&items_path)
        .await
        .map_err(|e| format!("读取 items.json 失败: {}", e))?;

    let items: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析 items.json 失败: {}", e))?;

    let config = serde_json::json!({
        "items": items,
    });

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 加载用户配置
#[tauri::command]
async fn load_user_config(
    game_path: String,
) -> Result<ConfigLoadResult<serde_json::Value>, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    let config_path = steam_settings_dir.join("configs.user.ini");

    if !config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&config_path)
        .await
        .map_err(|e| format!("读取 configs.user.ini 失败: {}", e))?;

    let mut username = String::new();
    let mut language = String::from("english");
    let mut steam_id = String::new();
    let mut saves_folder_name = String::new();
    let mut local_save_path = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "user_name" => username = value.to_string(),
                "language" => language = value.to_string(),
                "account_id" => steam_id = value.to_string(),
                "saves_folder_name" => saves_folder_name = value.to_string(),
                "local_save_path" => local_save_path = value.to_string(),
                _ => {}
            }
        }
    }

    // 检查头像文件
    let avatar_path = if steam_settings_dir.join("account_avatar.jpg").exists() {
        steam_settings_dir.join("account_avatar.jpg").to_string_lossy().to_string()
    } else if steam_settings_dir.join("account_avatar.png").exists() {
        steam_settings_dir.join("account_avatar.png").to_string_lossy().to_string()
    } else {
        String::new()
    };

    let config = serde_json::json!({
        "username": username,
        "language": language,
        "steamId": steam_id,
        "savesFolderName": saves_folder_name,
        "localSavePath": local_save_path,
        "avatarPath": avatar_path,
    });

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 加载主配置
#[tauri::command]
async fn load_main_config(
    game_path: String,
) -> Result<ConfigLoadResult<serde_json::Value>, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    let config_path = steam_settings_dir.join("configs.main.ini");

    if !config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&config_path)
        .await
        .map_err(|e| format!("读取 configs.main.ini 失败: {}", e))?;

    let mut new_app_ticket = false;
    let mut gc_token = false;
    let mut offline = false;
    let mut disable_networking = false;
    let mut ticket = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "new_app_ticket" => new_app_ticket = value == "1",
                "gc_token" => gc_token = value == "1",
                "offline" => offline = value == "1",
                "disable_networking" => disable_networking = value == "1",
                "ticket" => ticket = value.to_string(),
                _ => {}
            }
        }
    }

    let config = serde_json::json!({
        "newAppTicket": new_app_ticket,
        "gcToken": gc_token,
        "offlineMode": offline,
        "disableNetworking": disable_networking,
        "encryptedAppTicket": ticket,
    });

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 加载排行榜配置
#[tauri::command]
async fn load_leaderboards_config(
    game_path: String,
) -> Result<ConfigLoadResult<serde_json::Value>, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    let leaderboards_path = steam_settings_dir.join("leaderboards.txt");

    if !leaderboards_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&leaderboards_path)
        .await
        .map_err(|e| format!("读取 leaderboards.txt 失败: {}", e))?;

    let mut leaderboards = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() >= 3 {
            leaderboards.push(serde_json::json!({
                "name": parts[0],
                "sortMethod": parts[1].parse::<i64>().unwrap_or(0),
                "displayType": parts[2].parse::<i64>().unwrap_or(0),
            }));
        }
    }

    let config = serde_json::json!({
        "leaderboards": leaderboards,
    });

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 加载统计配置
#[tauri::command]
async fn load_stats_config(
    game_path: String,
) -> Result<ConfigLoadResult<serde_json::Value>, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    let stats_path = steam_settings_dir.join("stats.json");

    if !stats_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&stats_path)
        .await
        .map_err(|e| format!("读取 stats.json 失败: {}", e))?;

    let stats: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析 stats.json 失败: {}", e))?;

    // 读取 allow_unknown_stats
    let config_path = steam_settings_dir.join("configs.main.ini");
    let mut allow_unknown_stats = false;

    if config_path.exists() {
        let config_content = fs::read_to_string(&config_path).await.unwrap_or_default();
        for line in config_content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                if key.trim() == "allow_unknown_stats" {
                    allow_unknown_stats = value.trim() == "1";
                    break;
                }
            }
        }
    }

    let config = serde_json::json!({
        "stats": stats,
        "allowUnknownStats": allow_unknown_stats,
    });

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 加载 DLC 配置
#[tauri::command]
async fn load_dlc_config(
    game_path: String,
) -> Result<ConfigLoadResult<serde_json::Value>, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    let config_path = steam_settings_dir.join("configs.app.ini");

    let mut dlcs = Vec::new();
    let mut depots = Vec::new();
    let mut installed_apps = Vec::new();
    let mut unlock_all = true; // 默认解锁全部

    // 读取 configs.app.ini 中的 DLC
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .await
            .map_err(|e| format!("读取 configs.app.ini 失败: {}", e))?;

        let mut in_dlcs_section = false;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("[app::dlcs]") {
                in_dlcs_section = true;
            } else if in_dlcs_section && line.starts_with('[') && line.ends_with(']') {
                // 离开了 dlcs section
                in_dlcs_section = false;
            } else if in_dlcs_section {
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim();
                    
                    if key == "unlock_all" {
                        unlock_all = value == "1";
                    } else if let Ok(app_id) = key.parse::<i64>() {
                        // 这是DLC配置行：ID=name
                        let mut dlc_map = serde_json::Map::new();
                        dlc_map.insert("appId".to_string(), serde_json::json!(app_id));
                        dlc_map.insert("name".to_string(), serde_json::json!(value));
                        dlc_map.insert("key".to_string(), serde_json::json!(""));
                        dlcs.push(serde_json::Value::Object(dlc_map));
                    }
                }
            }
        }
    }

    // 读取 depots.txt
    let depots_path = steam_settings_dir.join("depots.txt");
    if depots_path.exists() {
        let content = fs::read_to_string(&depots_path).await.unwrap_or_default();
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() {
                if let Ok(id) = line.parse::<i64>() {
                    depots.push(serde_json::json!({"id": id}));
                }
            }
        }
    }

    // 读取 installed_app_ids.txt
    let apps_path = steam_settings_dir.join("installed_app_ids.txt");
    if apps_path.exists() {
        let content = fs::read_to_string(&apps_path).await.unwrap_or_default();
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() {
                if let Ok(id) = line.parse::<i64>() {
                    installed_apps.push(serde_json::json!({"id": id}));
                }
            }
        }
    }

    let config = serde_json::json!({
        "enabled": true,
        "unlockAll": unlock_all,
        "dlcs": dlcs,
        "depots": depots,
        "installedApps": installed_apps,
    });

    Ok(ConfigLoadResult {
        exists: dlcs.len() > 0 || depots.len() > 0 || installed_apps.len() > 0 || unlock_all,
        config: Some(config),
    })
}

/// 加载控制器配置
#[tauri::command]
async fn load_controller_config(
    game_path: String,
) -> Result<ConfigLoadResult<serde_json::Value>, String> {
    use tokio::fs;

    let controller_dir = Path::new(&game_path).join("steam_settings").join("controller");

    if !controller_dir.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let mut action_sets = Vec::new();

    let mut entries = fs::read_dir(&controller_dir).await
        .map_err(|e| format!("读取 controller 目录失败: {}", e))?;

    while let Some(entry) = entries.next_entry().await
        .map_err(|e| format!("读取目录条目失败: {}", e))? {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "txt" {
                    let name = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string();

                    let content = fs::read_to_string(&path).await.unwrap_or_default();
                    let mut digital_actions = Vec::new();
                    let mut analog_actions = Vec::new();

                    for line in content.lines() {
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }

                        let parts: Vec<&str> = line.split('=').collect();
                        if parts.len() == 2 {
                            // 数字动作: action_name=button
                            digital_actions.push(serde_json::json!({
                                "name": parts[0],
                                "button": parts[1],
                            }));
                        } else if parts.len() == 3 {
                            // 模拟动作: action_name=analog=mode
                            analog_actions.push(serde_json::json!({
                                "name": parts[0],
                                "analog": parts[1],
                                "mode": parts[2],
                            }));
                        }
                    }

                    action_sets.push(serde_json::json!({
                        "name": name,
                        "digitalActions": digital_actions,
                        "analogActions": analog_actions,
                    }));
                }
            }
        }
    }

    // 检查 glyphs 文件夹
    let glyphs_path = controller_dir.join("glyphs");
    let glyphs_folder = if glyphs_path.exists() && glyphs_path.is_dir() {
        glyphs_path.to_string_lossy().to_string()
    } else {
        String::new()
    };

    let config = serde_json::json!({
        "actionSets": action_sets,
        "glyphsFolder": glyphs_folder,
    });

    Ok(ConfigLoadResult {
        exists: action_sets.len() > 0,
        config: Some(config),
    })
}

/// 加载 Overlay 配置
#[tauri::command]
async fn load_overlay_config(
    game_path: String,
) -> Result<ConfigLoadResult<serde_json::Value>, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    let config_path = steam_settings_dir.join("configs.overlay.ini");

    if !config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&config_path)
        .await
        .map_err(|e| format!("读取 configs.overlay.ini 失败: {}", e))?;

    let mut enabled = false;
    let mut show_fps = false;
    let mut show_clock = false;
    let mut show_notifications = true;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "enable_experimental_overlay" => enabled = value == "1",
                "show_fps" => show_fps = value == "1",
                "show_clock" => show_clock = value == "1",
                "show_notifications" => show_notifications = value == "1",
                _ => {}
            }
        }
    }

    // 检查声音文件
    let sounds_dir = steam_settings_dir.join("sounds");
    let achievement_sound_path = sounds_dir.join("overlay_achievement_notification.wav");
    let friend_sound_path = sounds_dir.join("overlay_friend_notification.wav");

    // 检查字体文件
    let font_path = if steam_settings_dir.join("font.ttf").exists() {
        steam_settings_dir.join("font.ttf").to_string_lossy().to_string()
    } else if steam_settings_dir.join("font.otf").exists() {
        steam_settings_dir.join("font.otf").to_string_lossy().to_string()
    } else {
        String::new()
    };

    let config = serde_json::json!({
        "enabled": enabled,
        "showFPS": show_fps,
        "showClock": show_clock,
        "showNotifications": show_notifications,
        "achievementSoundPath": if achievement_sound_path.exists() { achievement_sound_path.to_string_lossy().to_string() } else { String::new() },
        "friendSoundPath": if friend_sound_path.exists() { friend_sound_path.to_string_lossy().to_string() } else { String::new() },
        "fontPath": font_path,
    });

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 配置加载结果
#[derive(serde::Serialize)]
pub struct ConfigLoadResult<T> {
    exists: bool,
    config: Option<T>,
}

/// 在系统默认浏览器中打开外部链接
#[tauri::command]
async fn open_external_link(url: String) -> Result<(), String> {
    use std::process::Command;

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/c", "start", "", &url])
            .spawn()
            .map_err(|e| format!("打开链接失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("打开链接失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("打开链接失败: {}", e))?;
    }

    Ok(())
}

/// 验证游戏ID是否合法（只允许数字）
fn validate_game_id(game_id: &str) -> Result<(), String> {
    if game_id.is_empty() {
        return Err("游戏ID不能为空".to_string());
    }
    // 只允许数字字符
    if !game_id.chars().all(|c| c.is_ascii_digit()) {
        return Err("游戏ID只能包含数字".to_string());
    }
    // 限制长度
    if game_id.len() > 20 {
        return Err("游戏ID长度不能超过20个字符".to_string());
    }
    Ok(())
}

/// 获取游戏封面图片的 base64 Data URL
/// 从 resources/pic/Game_Cover 目录读取游戏封面图片
#[tauri::command]
async fn get_game_cover(app: AppHandle, game_id: String) -> Result<String, String> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use tokio::fs;

    // 验证游戏ID，防止路径遍历攻击
    validate_game_id(&game_id)?;

    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;

    // 构建封面图片路径
    let cover_dir = resource_dir.join("pic").join("Game_Cover");
    let jpg_path = cover_dir.join(format!("{}.jpg", game_id));
    let png_path = cover_dir.join(format!("{}.png", game_id));
    let webp_path = cover_dir.join(format!("{}.webp", game_id));

    // 尝试读取不同格式的图片
    let (image_data, mime_type) = if jpg_path.exists() {
        let data = fs::read(&jpg_path).await.map_err(|e| format!("读取图片失败: {}", e))?;
        (data, "image/jpeg")
    } else if png_path.exists() {
        let data = fs::read(&png_path).await.map_err(|e| format!("读取图片失败: {}", e))?;
        (data, "image/png")
    } else if webp_path.exists() {
        let data = fs::read(&webp_path).await.map_err(|e| format!("读取图片失败: {}", e))?;
        (data, "image/webp")
    } else {
        return Ok(String::new()); // 图片不存在，返回空字符串
    };

    // 转换为 base64
    let base64_string = STANDARD.encode(&image_data);
    let data_url = format!("data:{};base64,{}", mime_type, base64_string);

    Ok(data_url)
}

/// 获取游戏补丁的 Readme 内容
/// 从 resources/crack/分类/Readme/游戏ID.txt 读取补丁说明
#[tauri::command]
async fn get_patch_readme(app: AppHandle, game_id: String, patch_type: u8) -> Result<String, String> {
    use tokio::fs;

    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;

    // 根据 patch_type 确定分类文件夹名称
    let category_folder = match patch_type {
        0 => "免_steam",
        1 => "局域网联机",
        2 => "steam_联机",
        3 => "D_加密虚拟机",
        _ => return Ok(String::new()), // 未知的 patch_type，返回空
    };

    // 构建 Readme 文件路径
    let readme_path = resource_dir
        .join("crack")
        .join(category_folder)
        .join("Readme")
        .join(format!("{}.txt", game_id));

    // 检查文件是否存在
    if !readme_path.exists() {
        return Ok(String::new()); // 文件不存在，返回空字符串
    }

    // 读取文件内容
    let content = fs::read_to_string(&readme_path)
        .await
        .map_err(|e| format!("读取 Readme 文件失败: {}", e))?;

    Ok(content)
}

/// 检查补丁文件是否存在
/// 返回本地补丁文件的完整路径，如果不存在则返回空字符串
#[tauri::command]
async fn check_patch_file_exists(app: AppHandle, patch_source_path: String) -> Result<String, String> {
    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;

    // 处理路径：如果 patch_source_path 已经包含 "resources/" 或 "Resources/" 前缀，则移除它
    let patch_lower = patch_source_path.to_lowercase();
    let patch_relative_path = if patch_lower.starts_with("resources/") {
        &patch_source_path[10..]  // 移除 "resources/" 前缀
    } else if patch_lower.starts_with("resources\\") {
        &patch_source_path[11..]  // 移除 "resources\" 前缀
    } else {
        &patch_source_path
    };

    // 自动添加 .7z 后缀（如果路径中没有）
    let patch_file_name = if patch_relative_path.ends_with(".7z") {
        patch_relative_path.to_string()
    } else {
        format!("{}.7z", patch_relative_path)
    };

    // 构建完整的补丁文件路径（相对于资源目录）
    let patch_file_path = resource_dir.join(&patch_file_name);

    if patch_file_path.exists() && patch_file_path.is_file() {
        Ok(patch_file_path.to_string_lossy().to_string())
    } else {
        Ok(String::new())  // 文件不存在，返回空字符串
    }
}

/// 从用户选择的压缩包文件应用补丁
/// 直接解压用户选择的7z文件到游戏目录
#[tauri::command]
async fn apply_patch_from_file(
    archive_path: String,
    game_path: String,
) -> Result<ApplyPatchResult, String> {
    let archive_path = Path::new(&archive_path);
    let target_path = Path::new(&game_path);

    if !archive_path.exists() {
        return Err(format!("补丁文件不存在: {}", archive_path.display()));
    }

    if !target_path.exists() {
        return Err(format!("游戏目标路径不存在: {}", game_path));
    }

    // 创建临时解压目录
    let temp_dir = std::env::temp_dir().join(format!("steam_tool_patch_{}", std::process::id()));
    let temp_path = temp_dir.clone();

    // 清理并创建临时目录
    if temp_path.exists() {
        let _ = tokio::fs::remove_dir_all(&temp_path).await;
    }
    tokio::fs::create_dir_all(&temp_path)
        .await
        .map_err(|e| format!("创建临时目录失败: {}", e))?;

    // 解压7z文件
    match extract_7z(&archive_path, &temp_path).await {
        Ok(_) => {}
        Err(e) => {
            // 清理临时目录
            let _ = tokio::fs::remove_dir_all(&temp_path).await;
            return Err(format!("解压补丁失败: {}", e));
        }
    }

    let mut backed_up_files: Vec<String> = Vec::new();
    let mut copied_files: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    // 递归复制解压后的文件
    match copy_dir_with_backup(&temp_path, target_path, &mut backed_up_files, &mut copied_files, &mut errors).await {
        Ok(_) => {}
        Err(e) => {
            errors.push(e);
        }
    }

    // 清理临时目录
    let _ = tokio::fs::remove_dir_all(&temp_path).await;

    Ok(ApplyPatchResult {
        success: errors.is_empty(),
        backed_up_files,
        copied_files,
        errors,
    })
}

/// 进度文件信息
#[derive(serde::Serialize)]
pub struct ProgressFileInfo {
    pub name: String,
    pub path: String,
}

/// 获取下载进度文件列表
/// 扫描程序根目录下的 {百分比}% - {depotId}.json 文件
#[tauri::command]
async fn get_download_progress_files(_app: AppHandle) -> Result<Vec<ProgressFileInfo>, String> {
    // 获取程序所在目录（PublicOut 目录）
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("无法获取程序路径: {}", e))?
        .parent()
        .ok_or("无法获取程序所在目录")?
        .to_path_buf();

    let mut progress_files = Vec::new();

    // 读取目录内容
    let entries = std::fs::read_dir(&exe_dir).map_err(|e| format!("读取目录失败: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
        let file_path = entry.path();

        if file_path.is_file() {
            if let Some(file_name) = file_path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                // 匹配进度文件格式: "{百分比}% - {depotId}.json"
                if file_name_str.ends_with(".json") && file_name_str.contains("% - ") {
                    progress_files.push(ProgressFileInfo {
                        name: file_name_str.to_string(),
                        path: file_path.to_string_lossy().to_string(),
                    });
                }
            }
        }
    }

    Ok(progress_files)
}

/// 读取 JSON 文件内容
#[tauri::command]
async fn read_json_file(file_path: String) -> Result<serde_json::Value, String> {
    let path = Path::new(&file_path);

    if !path.exists() {
        return Err("文件不存在".to_string());
    }

    let content = std::fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))?;
    let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    Ok(json)
}

/// 目录项信息
#[derive(Debug, Clone, serde::Serialize)]
struct DirEntry {
    name: String,
    path: String,
    is_dir: bool,
}

/// 读取目录内容
#[tauri::command]
async fn read_directory(path: String) -> Result<Vec<DirEntry>, String> {
    let dir_path = Path::new(&path);

    if !dir_path.exists() {
        return Err("目录不存在".to_string());
    }

    if !dir_path.is_dir() {
        return Err("路径不是目录".to_string());
    }

    let mut entries = Vec::new();

    for entry in std::fs::read_dir(dir_path).map_err(|e| format!("读取目录失败: {}", e))? {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let metadata = entry.metadata().map_err(|e| format!("获取元数据失败: {}", e))?;

        entries.push(DirEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path().to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
        });
    }

    Ok(entries)
}

/// 关闭系统
#[tauri::command]
async fn shutdown_system() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        // 使用 shutdown 命令关闭计算机
        Command::new("shutdown")
            .args(&["/s", "/t", "0"])
            .spawn()
            .map_err(|e| format!("执行关机命令失败: {}", e))?;

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("当前仅支持 Windows 系统".to_string())
    }
}

/// 删除文件
#[tauri::command]
async fn delete_file(file_path: String) -> Result<(), String> {
    let path = Path::new(&file_path);

    if !path.exists() {
        return Err("文件不存在".to_string());
    }

    std::fs::remove_file(path).map_err(|e| format!("删除文件失败: {}", e))?;

    Ok(())
}

/// 运行 Tauri 应用程序
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(setup_app)
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            select_folder,
            select_exe_file,
            apply_patch,
            read_manifest_folder,
            read_text_file,
            get_available_drive,
            get_manifest_path,
            start_game_download,
            read_games_config,
            unpack_game_exe,
            apply_steam_patch_basic,
            save_lan_multiplayer_config,
            save_achievements_config,
            save_items_config,
            save_user_config,
            save_main_config,
            save_leaderboards_config,
            save_stats_config,
            save_dlc_config,
            scan_dlc_folder,
            save_controller_config,
            save_overlay_config,
            load_lan_multiplayer_config,
            load_achievements_config,
            load_items_config,
            load_user_config,
            load_main_config,
            load_leaderboards_config,
            load_stats_config,
            load_dlc_config,
            load_controller_config,
            load_overlay_config,
            open_external_link,
            get_game_cover,
            get_patch_readme,
            check_patch_file_exists,
            apply_patch_from_file,
            get_download_progress_files,
            read_json_file,
            read_directory,
            shutdown_system,
            delete_file,
        ])
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用程序时发生错误");
}
