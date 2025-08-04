use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use dirs;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Language {
    Zh,
    En,
}

impl Default for Language {
    fn default() -> Self {
        Language::Zh // 默认中文
    }
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "zh" | "zh-cn" | "chinese" => Language::Zh,
            "en" | "en-us" | "english" => Language::En,
            _ => Language::Zh,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Language::Zh => "zh",
            Language::En => "en",
        }.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationFile {
    pub language: String,
    pub messages: HashMap<String, String>,
}

pub struct I18n {
    current_language: Language,
    messages: HashMap<Language, HashMap<String, String>>,
}

impl I18n {
    pub fn new(language: Language) -> Self {
        let mut i18n = Self {
            current_language: language,
            messages: HashMap::new(),
        };
        i18n.load_translations_from_files();
        // 如果文件加载失败，回退到内置翻译
        if i18n.messages.is_empty() {
            i18n.initialize_fallback_messages();
        }
        i18n
    }

    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    pub fn get_language(&self) -> &Language {
        &self.current_language
    }

    pub fn t(&self, key: &str) -> String {
        self.messages
            .get(&self.current_language)
            .and_then(|translations| translations.get(key))
            .or_else(|| {
                // 如果当前语言没有翻译，尝试英文
                self.messages
                    .get(&Language::En)
                    .and_then(|translations| translations.get(key))
            })
            .or_else(|| {
                // 如果英文也没有，尝试中文
                self.messages
                    .get(&Language::Zh)
                    .and_then(|translations| translations.get(key))
            })
            .cloned()
            .unwrap_or_else(|| format!("Missing translation: {}", key))
    }

    pub fn t_with_args(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut message = self.t(key);
        for (placeholder, value) in args {
            message = message.replace(&format!("{{{}}}", placeholder), value);
        }
        message
    }

    fn get_translations_dir() -> Option<PathBuf> {
        if let Some(home_dir) = dirs::home_dir() {
            Some(home_dir.join(".claude").join("translations"))
        } else {
            None
        }
    }

    fn load_translations_from_files(&mut self) {
        if let Some(translations_dir) = Self::get_translations_dir() {
            if !translations_dir.exists() {
                if let Err(e) = fs::create_dir_all(&translations_dir) {
                    eprintln!("Failed to create translations directory: {}", e);
                    return;
                }
                // 创建默认翻译文件
                self.create_default_translation_files(&translations_dir);
            }

            // 加载所有语言的翻译文件
            for language in [Language::Zh, Language::En] {
                let file_path = translations_dir.join(format!("backend_{}.json", language.to_string()));
                if let Ok(content) = fs::read_to_string(&file_path) {
                    if let Ok(translation_file) = serde_json::from_str::<TranslationFile>(&content) {
                        self.messages.insert(language, translation_file.messages);
                    }
                }
            }
        }
    }

    fn create_default_translation_files(&self, translations_dir: &PathBuf) {
        let zh_translations = self.get_default_zh_translations();
        let en_translations = self.get_default_en_translations();

        let zh_file = TranslationFile {
            language: "zh".to_string(),
            messages: zh_translations,
        };

        let en_file = TranslationFile {
            language: "en".to_string(),
            messages: en_translations,
        };

        if let Ok(zh_content) = serde_json::to_string_pretty(&zh_file) {
            let _ = fs::write(translations_dir.join("backend_zh.json"), zh_content);
        }

        if let Ok(en_content) = serde_json::to_string_pretty(&en_file) {
            let _ = fs::write(translations_dir.join("backend_en.json"), en_content);
        }
    }

    fn get_default_zh_translations(&self) -> HashMap<String, String> {
        let mut translations = HashMap::new();
        
        // Provider messages
        translations.insert("provider.home_dir_not_found".to_string(), "无法获取用户主目录".to_string());
        translations.insert("provider.create_config_dir_failed".to_string(), "无法创建配置目录: {error}".to_string());
        translations.insert("provider.read_claude_settings_failed".to_string(), "读取 Claude settings 文件失败: {error}".to_string());
        translations.insert("provider.parse_claude_settings_failed".to_string(), "解析 Claude settings 文件失败: {error}".to_string());
        translations.insert("provider.serialize_claude_settings_failed".to_string(), "序列化 Claude settings 失败: {error}".to_string());
        translations.insert("provider.write_claude_settings_failed".to_string(), "写入 Claude settings 文件失败: {error}".to_string());
        translations.insert("provider.read_config_failed".to_string(), "读取配置文件失败: {error}".to_string());
        translations.insert("provider.parse_config_failed".to_string(), "解析配置文件失败: {error}".to_string());
        translations.insert("provider.serialize_config_failed".to_string(), "序列化配置失败: {error}".to_string());
        translations.insert("provider.write_config_failed".to_string(), "写入配置文件失败: {error}".to_string());
        translations.insert("provider.invalid_config_format".to_string(), "配置文件格式错误: {error}".to_string());
        translations.insert("provider.id_already_exists".to_string(), "ID '{id}' 已存在，请使用不同的ID".to_string());
        translations.insert("provider.add_success".to_string(), "成功添加代理商配置: {name}".to_string());
        translations.insert("provider.config_not_found".to_string(), "未找到ID为 '{id}' 的配置".to_string());
        translations.insert("provider.update_success".to_string(), "成功更新代理商配置: {name}".to_string());
        translations.insert("provider.delete_success".to_string(), "成功删除代理商配置: {name}".to_string());
        translations.insert("provider.switch_success".to_string(), "已成功切换到 {name} ({description})，配置已保存到 Raw Settings".to_string());
        translations.insert("provider.clear_success".to_string(), "已清理所有 ANTHROPIC 环境变量在 Raw Settings 中".to_string());
        translations.insert("provider.connection_test_complete".to_string(), "连接测试完成：{url}".to_string());
        
        // Process termination messages
        translations.insert("process.terminating_claude_processes".to_string(), "正在终止所有Claude进程以应用新的代理商配置...".to_string());
        translations.insert("process.found_active_sessions".to_string(), "找到 {count} 个活动的Claude会话".to_string());
        translations.insert("process.terminating_session".to_string(), "正在终止Claude会话: session_id={session_id}, run_id={run_id}, PID={pid}".to_string());
        translations.insert("process.session_terminated".to_string(), "成功终止Claude会话 {run_id}".to_string());
        translations.insert("process.session_terminate_false".to_string(), "终止Claude会话 {run_id} 返回false".to_string());
        translations.insert("process.force_terminate_failed".to_string(), "强制终止进程失败: {error}".to_string());
        translations.insert("process.session_terminate_failed".to_string(), "终止Claude会话 {run_id} 失败: {error}".to_string());
        translations.insert("process.force_terminate_also_failed".to_string(), "强制终止进程也失败: {error}".to_string());
        translations.insert("process.get_sessions_failed".to_string(), "获取Claude会话列表失败: {error}".to_string());
        translations.insert("process.termination_complete".to_string(), "Claude进程终止操作完成".to_string());

        // Storage messages
        translations.insert("storage.db_connection_failed".to_string(), "数据库连接失败: {error}".to_string());
        translations.insert("storage.query_execution_failed".to_string(), "查询执行失败: {error}".to_string());
        translations.insert("storage.table_not_found".to_string(), "表 '{table}' 不存在".to_string());
        translations.insert("storage.invalid_sql_query".to_string(), "无效的SQL查询".to_string());
        
        // MCP messages
        translations.insert("mcp.server_not_found".to_string(), "服务器 '{name}' 未找到".to_string());
        translations.insert("mcp.server_start_failed".to_string(), "启动MCP服务器失败: {error}".to_string());
        translations.insert("mcp.server_stop_failed".to_string(), "停止MCP服务器失败: {error}".to_string());
        translations.insert("mcp.server_add_success".to_string(), "成功添加MCP服务器: {name}".to_string());
        translations.insert("mcp.server_delete_success".to_string(), "成功删除MCP服务器: {name}".to_string());
        translations.insert("mcp.invalid_server_config".to_string(), "无效的服务器配置".to_string());
        
        // Agent messages
        translations.insert("agent.not_found".to_string(), "智能体 '{name}' 未找到".to_string());
        translations.insert("agent.create_success".to_string(), "成功创建智能体: {name}".to_string());
        translations.insert("agent.delete_success".to_string(), "成功删除智能体: {name}".to_string());
        translations.insert("agent.execution_failed".to_string(), "智能体执行失败: {error}".to_string());
        
        // Claude messages
        translations.insert("claude.binary_not_found".to_string(), "未找到Claude二进制文件".to_string());
        translations.insert("claude.project_not_found".to_string(), "项目未找到: {path}".to_string());
        translations.insert("claude.session_start_failed".to_string(), "启动Claude会话失败: {error}".to_string());
        translations.insert("claude.session_not_found".to_string(), "会话未找到: {session_id}".to_string());
        
        // Clipboard messages
        translations.insert("clipboard.image_save_failed".to_string(), "保存剪贴板图片失败: {error}".to_string());
        translations.insert("clipboard.no_image_data".to_string(), "剪贴板中没有图片数据".to_string());
        
        // Usage messages
        translations.insert("usage.stats_load_failed".to_string(), "加载使用统计失败: {error}".to_string());
        translations.insert("usage.stats_save_failed".to_string(), "保存使用统计失败: {error}".to_string());
        
        // Slash commands messages
        translations.insert("slash.command_not_found".to_string(), "斜杠命令未找到: {command}".to_string());
        translations.insert("slash.command_execution_failed".to_string(), "斜杠命令执行失败: {error}".to_string());
        translations.insert("slash.command_add_success".to_string(), "成功添加斜杠命令: {name}".to_string());
        translations.insert("slash.command_delete_success".to_string(), "成功删除斜杠命令: {name}".to_string());

        // Relay stations messages
        translations.insert("relay.station_not_found".to_string(), "中转站未找到".to_string());
        translations.insert("relay.station_add_success".to_string(), "中转站添加成功".to_string());
        translations.insert("relay.station_update_success".to_string(), "中转站更新成功".to_string());
        translations.insert("relay.station_delete_success".to_string(), "中转站删除成功".to_string());
        translations.insert("relay.manager_not_initialized".to_string(), "中转站管理器未初始化".to_string());
        translations.insert("relay.lock_error".to_string(), "锁定错误: {error}".to_string());
        translations.insert("relay.failed_to_list_stations".to_string(), "获取中转站列表失败: {error}".to_string());
        translations.insert("relay.failed_to_get_station".to_string(), "获取中转站失败: {error}".to_string());
        translations.insert("relay.failed_to_add_station".to_string(), "添加中转站失败: {error}".to_string());
        translations.insert("relay.failed_to_update_station".to_string(), "更新中转站失败: {error}".to_string());
        translations.insert("relay.failed_to_delete_station".to_string(), "删除中转站失败: {error}".to_string());
        translations.insert("relay.failed_to_get_station_info".to_string(), "获取中转站信息失败: {error}".to_string());
        translations.insert("relay.failed_to_list_tokens".to_string(), "获取令牌列表失败: {error}".to_string());
        translations.insert("relay.failed_to_create_token".to_string(), "创建令牌失败: {error}".to_string());
        translations.insert("relay.failed_to_update_token".to_string(), "更新令牌失败: {error}".to_string());
        translations.insert("relay.failed_to_delete_token".to_string(), "删除令牌失败: {error}".to_string());
        translations.insert("relay.token_delete_success".to_string(), "令牌删除成功".to_string());
        translations.insert("relay.failed_to_get_user_info".to_string(), "获取用户信息失败: {error}".to_string());
        translations.insert("relay.failed_to_get_logs".to_string(), "获取日志失败: {error}".to_string());
        translations.insert("relay.failed_to_test_connection".to_string(), "连接测试失败: {error}".to_string());
        translations.insert("relay.failed_to_get_user_groups".to_string(), "获取用户组失败: {error}".to_string());
        translations.insert("relay.failed_to_toggle_token".to_string(), "切换令牌状态失败: {error}".to_string());
        translations.insert("relay.failed_to_save_config".to_string(), "保存配置失败: {error}".to_string());
        translations.insert("relay.config_save_success".to_string(), "配置保存成功".to_string());
        translations.insert("relay.failed_to_get_config".to_string(), "获取配置失败: {error}".to_string());
        translations.insert("relay.failed_to_get_usage_status".to_string(), "获取使用状态失败: {error}".to_string());
        translations.insert("relay.failed_to_record_usage".to_string(), "记录使用失败: {error}".to_string());
        translations.insert("relay.usage_record_updated".to_string(), "使用记录已更新".to_string());
        translations.insert("relay.default_endpoint".to_string(), "默认端点".to_string());
        translations.insert("relay.current_configured_endpoint".to_string(), "当前配置的端点".to_string());

        translations
    }

    fn get_default_en_translations(&self) -> HashMap<String, String> {
        let mut translations = HashMap::new();
        
        // Provider messages
        translations.insert("provider.home_dir_not_found".to_string(), "Failed to get user home directory".to_string());
        translations.insert("provider.create_config_dir_failed".to_string(), "Failed to create config directory: {error}".to_string());
        translations.insert("provider.read_claude_settings_failed".to_string(), "Failed to read Claude settings file: {error}".to_string());
        translations.insert("provider.parse_claude_settings_failed".to_string(), "Failed to parse Claude settings file: {error}".to_string());
        translations.insert("provider.serialize_claude_settings_failed".to_string(), "Failed to serialize Claude settings: {error}".to_string());
        translations.insert("provider.write_claude_settings_failed".to_string(), "Failed to write Claude settings file: {error}".to_string());
        translations.insert("provider.read_config_failed".to_string(), "Failed to read config file: {error}".to_string());
        translations.insert("provider.parse_config_failed".to_string(), "Failed to parse config file: {error}".to_string());
        translations.insert("provider.serialize_config_failed".to_string(), "Failed to serialize config: {error}".to_string());
        translations.insert("provider.write_config_failed".to_string(), "Failed to write config file: {error}".to_string());
        translations.insert("provider.invalid_config_format".to_string(), "Invalid config file format: {error}".to_string());
        translations.insert("provider.id_already_exists".to_string(), "ID '{id}' already exists, please use a different ID".to_string());
        translations.insert("provider.add_success".to_string(), "Successfully added provider config: {name}".to_string());
        translations.insert("provider.config_not_found".to_string(), "Config with ID '{id}' not found".to_string());
        translations.insert("provider.update_success".to_string(), "Successfully updated provider config: {name}".to_string());
        translations.insert("provider.delete_success".to_string(), "Successfully deleted provider config: {name}".to_string());
        translations.insert("provider.switch_success".to_string(), "Successfully switched to {name} ({description}), config saved to Raw Settings".to_string());
        translations.insert("provider.clear_success".to_string(), "Cleared all ANTHROPIC environment variables in Raw Settings".to_string());
        translations.insert("provider.connection_test_complete".to_string(), "Connection test completed: {url}".to_string());
        
        // Process termination messages
        translations.insert("process.terminating_claude_processes".to_string(), "Terminating all Claude processes to apply new provider configuration...".to_string());
        translations.insert("process.found_active_sessions".to_string(), "Found {count} active Claude sessions".to_string());
        translations.insert("process.terminating_session".to_string(), "Terminating Claude session: session_id={session_id}, run_id={run_id}, PID={pid}".to_string());
        translations.insert("process.session_terminated".to_string(), "Successfully terminated Claude session {run_id}".to_string());
        translations.insert("process.session_terminate_false".to_string(), "Terminating Claude session {run_id} returned false".to_string());
        translations.insert("process.force_terminate_failed".to_string(), "Failed to force terminate process: {error}".to_string());
        translations.insert("process.session_terminate_failed".to_string(), "Failed to terminate Claude session {run_id}: {error}".to_string());
        translations.insert("process.force_terminate_also_failed".to_string(), "Force terminate process also failed: {error}".to_string());
        translations.insert("process.get_sessions_failed".to_string(), "Failed to get Claude sessions list: {error}".to_string());
        translations.insert("process.termination_complete".to_string(), "Claude process termination operation completed".to_string());

        // Storage messages
        translations.insert("storage.db_connection_failed".to_string(), "Database connection failed: {error}".to_string());
        translations.insert("storage.query_execution_failed".to_string(), "Query execution failed: {error}".to_string());
        translations.insert("storage.table_not_found".to_string(), "Table '{table}' does not exist".to_string());
        translations.insert("storage.invalid_sql_query".to_string(), "Invalid SQL query".to_string());
        
        // MCP messages
        translations.insert("mcp.server_not_found".to_string(), "Server '{name}' not found".to_string());
        translations.insert("mcp.server_start_failed".to_string(), "Failed to start MCP server: {error}".to_string());
        translations.insert("mcp.server_stop_failed".to_string(), "Failed to stop MCP server: {error}".to_string());
        translations.insert("mcp.server_add_success".to_string(), "Successfully added MCP server: {name}".to_string());
        translations.insert("mcp.server_delete_success".to_string(), "Successfully deleted MCP server: {name}".to_string());
        translations.insert("mcp.invalid_server_config".to_string(), "Invalid server configuration".to_string());
        
        // Agent messages
        translations.insert("agent.not_found".to_string(), "Agent '{name}' not found".to_string());
        translations.insert("agent.create_success".to_string(), "Successfully created agent: {name}".to_string());
        translations.insert("agent.delete_success".to_string(), "Successfully deleted agent: {name}".to_string());
        translations.insert("agent.execution_failed".to_string(), "Agent execution failed: {error}".to_string());
        
        // Claude messages
        translations.insert("claude.binary_not_found".to_string(), "Claude binary not found".to_string());
        translations.insert("claude.project_not_found".to_string(), "Project not found: {path}".to_string());
        translations.insert("claude.session_start_failed".to_string(), "Failed to start Claude session: {error}".to_string());
        translations.insert("claude.session_not_found".to_string(), "Session not found: {session_id}".to_string());
        
        // Clipboard messages
        translations.insert("clipboard.image_save_failed".to_string(), "Failed to save clipboard image: {error}".to_string());
        translations.insert("clipboard.no_image_data".to_string(), "No image data in clipboard".to_string());
        
        // Usage messages
        translations.insert("usage.stats_load_failed".to_string(), "Failed to load usage statistics: {error}".to_string());
        translations.insert("usage.stats_save_failed".to_string(), "Failed to save usage statistics: {error}".to_string());
        
        // Slash commands messages
        translations.insert("slash.command_not_found".to_string(), "Slash command not found: {command}".to_string());
        translations.insert("slash.command_execution_failed".to_string(), "Slash command execution failed: {error}".to_string());
        translations.insert("slash.command_add_success".to_string(), "Successfully added slash command: {name}".to_string());
        translations.insert("slash.command_delete_success".to_string(), "Successfully deleted slash command: {name}".to_string());

        // Relay stations messages
        translations.insert("relay.station_not_found".to_string(), "Station not found".to_string());
        translations.insert("relay.station_add_success".to_string(), "Station added successfully".to_string());
        translations.insert("relay.station_update_success".to_string(), "Station updated successfully".to_string());
        translations.insert("relay.station_delete_success".to_string(), "Station deleted successfully".to_string());
        translations.insert("relay.manager_not_initialized".to_string(), "Relay station manager not initialized".to_string());
        translations.insert("relay.lock_error".to_string(), "Lock error: {error}".to_string());
        translations.insert("relay.failed_to_list_stations".to_string(), "Failed to list stations: {error}".to_string());
        translations.insert("relay.failed_to_get_station".to_string(), "Failed to get station: {error}".to_string());
        translations.insert("relay.failed_to_add_station".to_string(), "Failed to add station: {error}".to_string());
        translations.insert("relay.failed_to_update_station".to_string(), "Failed to update station: {error}".to_string());
        translations.insert("relay.failed_to_delete_station".to_string(), "Failed to delete station: {error}".to_string());
        translations.insert("relay.failed_to_get_station_info".to_string(), "Failed to get station info: {error}".to_string());
        translations.insert("relay.failed_to_list_tokens".to_string(), "Failed to list tokens: {error}".to_string());
        translations.insert("relay.failed_to_create_token".to_string(), "Failed to create token: {error}".to_string());
        translations.insert("relay.failed_to_update_token".to_string(), "Failed to update token: {error}".to_string());
        translations.insert("relay.failed_to_delete_token".to_string(), "Failed to delete token: {error}".to_string());
        translations.insert("relay.token_delete_success".to_string(), "Token deleted successfully".to_string());
        translations.insert("relay.failed_to_get_user_info".to_string(), "Failed to get user info: {error}".to_string());
        translations.insert("relay.failed_to_get_logs".to_string(), "Failed to get logs: {error}".to_string());
        translations.insert("relay.failed_to_test_connection".to_string(), "Failed to test connection: {error}".to_string());
        translations.insert("relay.failed_to_get_user_groups".to_string(), "Failed to get user groups: {error}".to_string());
        translations.insert("relay.failed_to_toggle_token".to_string(), "Failed to toggle token: {error}".to_string());
        translations.insert("relay.failed_to_save_config".to_string(), "Failed to save config: {error}".to_string());
        translations.insert("relay.config_save_success".to_string(), "Configuration saved successfully".to_string());
        translations.insert("relay.failed_to_get_config".to_string(), "Failed to get config: {error}".to_string());
        translations.insert("relay.failed_to_get_usage_status".to_string(), "Failed to get usage status: {error}".to_string());
        translations.insert("relay.failed_to_record_usage".to_string(), "Failed to record usage: {error}".to_string());
        translations.insert("relay.usage_record_updated".to_string(), "Usage record updated".to_string());
        translations.insert("relay.default_endpoint".to_string(), "Default Endpoint".to_string());
        translations.insert("relay.current_configured_endpoint".to_string(), "Current configured endpoint".to_string());

        translations
    }

    fn initialize_fallback_messages(&mut self) {
        let zh_translations = self.get_default_zh_translations();
        let en_translations = self.get_default_en_translations();
        
        self.messages.insert(Language::Zh, zh_translations);
        self.messages.insert(Language::En, en_translations);
    }

    /// 重新加载翻译文件
    pub fn reload_translations(&mut self) {
        self.messages.clear();
        self.load_translations_from_files();
        if self.messages.is_empty() {
            self.initialize_fallback_messages();
        }
    }

    /// 保存当前翻译到文件
    pub fn save_translations_to_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(translations_dir) = Self::get_translations_dir() {
            if !translations_dir.exists() {
                fs::create_dir_all(&translations_dir)?;
            }

            for (language, messages) in &self.messages {
                let translation_file = TranslationFile {
                    language: language.to_string(),
                    messages: messages.clone(),
                };

                let content = serde_json::to_string_pretty(&translation_file)?;
                let file_path = translations_dir.join(format!("backend_{}.json", language.to_string()));
                fs::write(file_path, content)?;
            }
        }
        Ok(())
    }
}

// 全局单例实例
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

static GLOBAL_I18N: Lazy<Arc<Mutex<I18n>>> = Lazy::new(|| {
    Arc::new(Mutex::new(I18n::new(Language::Zh)))
});

pub fn set_language(language: Language) {
    if let Ok(mut i18n) = GLOBAL_I18N.lock() {
        i18n.set_language(language);
    }
}

pub fn get_language() -> Language {
    GLOBAL_I18N.lock()
        .map(|i18n| i18n.get_language().clone())
        .unwrap_or_default()
}

pub fn t(key: &str) -> String {
    GLOBAL_I18N.lock()
        .map(|i18n| i18n.t(key))
        .unwrap_or_else(|_| format!("Translation error: {}", key))
}

pub fn t_with_args(key: &str, args: &[(&str, &str)]) -> String {
    GLOBAL_I18N.lock()
        .map(|i18n| i18n.t_with_args(key, args))
        .unwrap_or_else(|_| format!("Translation error: {}", key))
}

// 便捷宏
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        crate::i18n::t($key)
    };
    ($key:expr, $($arg_name:expr => $arg_value:expr),+) => {
        crate::i18n::t_with_args($key, &[$(($arg_name, $arg_value)),+])
    };
}