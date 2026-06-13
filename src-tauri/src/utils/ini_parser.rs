// 通用 INI 解析器
// 消除多个模块中重复 INI 解析代码
// 使用方法：
//   1. 创建 IniParser 实例并解析原始内容
//   2. 使用 iter_section() 遍历 section
//   3. 使用 get_bool()/get_string()/get_f64()/get_i32()/get_u32() 提取值
// TODO: 后续重构时替换 config_core、game_features 等模块中的重复 INI 解析代码

#![allow(dead_code)]

use std::collections::HashMap;

/// INI 解析结果：section名 -> [(key, value)]
type IniData = HashMap<String, Vec<(String, String)>>;

/// 通用 INI 解析器
pub struct IniParser {
    data: IniData,
}

impl IniParser {
    /// 解析 INI 格式的内容
    /// 支持：
    ///   - 注释行（# 开头）
    ///   - [section] 标题
    ///   - key = value 条目
    ///   - key = value # 注释（行尾注释）
    ///   - 空行自动跳过
    pub fn new(content: &str) -> Self {
        let mut data: IniData = HashMap::new();
        let mut current_section = String::new();

        for line in content.lines() {
            let line = line.trim();

            // 跳过空行和注释
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // 解析 section 标题
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                continue;
            }

            // 解析 key=value
            if let Some((key, raw_value)) = line.split_once('=') {
                let key = key.trim().to_string();
                // 去除行尾注释（# 后面的内容）
                let value = if let Some(pos) = raw_value.find('#') {
                    raw_value[..pos].trim().to_string()
                } else {
                    raw_value.trim().to_string()
                };

                data.entry(current_section.clone())
                    .or_default()
                    .push((key, value));
            }
        }

        Self { data }
    }

    /// 获取指定 section 下的所有 key-value 对
    pub fn get_section(&self, section: &str) -> Option<&[(String, String)]> {
        self.data
            .get(section)
            .map(|v| v.as_slice())
    }

    /// 检查 section 是否存在
    pub fn has_section(&self, section: &str) -> bool {
        self.data.contains_key(section)
    }

    /// 获取指定 section+key 的字符串值
    pub fn get_string(&self, section: &str, key: &str) -> Option<String> {
        self.data
            .get(section)?
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    }

    /// 获取指定 section+key 的 bool 值（支持 "1"/"0"/"true"/"false"）
    pub fn get_bool(&self, section: &str, key: &str) -> Option<bool> {
        self.get_string(section, key).map(|v| {
            v == "1" || v.eq_ignore_ascii_case("true")
        })
    }

    /// 获取指定 section+key 的 i32 值
    pub fn get_i32(&self, section: &str, key: &str) -> Option<i32> {
        self.get_string(section, key)?.parse().ok()
    }

    /// 获取指定 section+key 的 u32 值
    pub fn get_u32(&self, section: &str, key: &str) -> Option<u32> {
        self.get_string(section, key)?.parse().ok()
    }

    /// 获取指定 section+key 的 f64 值
    pub fn get_f64(&self, section: &str, key: &str) -> Option<f64> {
        self.get_string(section, key)?.parse().ok()
    }

    /// 遍历所有 section 执行回调
    pub fn for_each_section<F>(&self, mut f: F)
    where
        F: FnMut(&str, &[(String, String)]),
    {
        for (section, pairs) in &self.data {
            f(section, pairs.as_slice());
        }
    }

    /// 遍历指定 section 下的所有 key-value 对执行回调
    pub fn for_each_in_section<F>(&self, section: &str, mut f: F)
    where
        F: FnMut(&str, &str),
    {
        if let Some(pairs) = self.data.get(section) {
            for (k, v) in pairs {
                f(k, v);
            }
        }
    }

    /// 获取所有 section 名
    pub fn sections(&self) -> Vec<&str> {
        self.data.keys().map(|s| s.as_str()).collect()
    }
}

// ============================================
// INI 生成工具
// ============================================

/// INI 构建器，用于程序化生成 INI 格式字符串
pub struct IniBuilder {
    lines: Vec<String>,
}

impl IniBuilder {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    /// 添加注释行
    pub fn comment(mut self, text: &str) -> Self {
        self.lines.push(format!("# {}", text));
        self
    }

    /// 添加空行
    pub fn blank_line(mut self) -> Self {
        self.lines.push(String::new());
        self
    }

    /// 开始一个 section
    pub fn section(mut self, name: &str) -> Self {
        self.lines.push(format!("[{}]", name));
        self
    }

    /// 添加 key=value 条目（自动处理值中的注释）
    pub fn entry(mut self, key: &str, value: impl std::fmt::Display) -> Self {
        self.lines.push(format!("{} = {}", key, value));
        self
    }

    /// 添加 bool 值（输出 0/1）
    pub fn bool_entry(mut self, key: &str, value: bool) -> Self {
        self.lines.push(format!("{} = {}", key, value as i32));
        self
    }

    /// 添加带注释的条目
    pub fn entry_with_comment(mut self, key: &str, value: impl std::fmt::Display, comment: &str) -> Self {
        self.lines.push(format!("{} = {} # {}", key, value, comment));
        self
    }

    /// 生成最终的 INI 字符串
    pub fn build(self) -> String {
        self.lines.join("\n")
    }
}

impl Default for IniBuilder {
    fn default() -> Self {
        Self::new()
    }
}