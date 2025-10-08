/// 登录token过期时间，单位：秒，默认7天
pub const ACCESS_TOKEN_EXPIRES_IN: u64 = 60 * 60 * 24 * 7;

/// 助手默认角色提示语
#[allow(unused)]
pub const ASSISTANT_DEFAULT_ROLE_PROMPT: &str = "你是一个乐于助人的助手";

/// 超级管理员角色编码
pub const SUPER_ADMIN_ROLE: (i64, &str) = (1, "sa");

/// 管理员角色编码
#[allow(unused)]
pub const ADMIN_ROLE: (i64, &str) = (2, "admin");

/// 用户角色编码
pub const USER_ROLE: (i64, &str) = (3, "user");

/// 向量数据库文本检索指令
pub const TEXT_SEARCH_INSTRUCTION: &str = "为这个句子生成表示以用于检索相关文章：";

/// MCP服务工具名称分隔符。注意不能定义特殊字符，因为有些模型不支持，仅可定义为字母、数字、下划线、横线。
pub const MCP_SERVER_TOOL_NAME_SEPARATOR: &str = "A-_-A";

/// MCP工具调用深度限制。超过此深度将终止调用。
#[allow(unused)]
pub const MAX_MCP_TOOL_INVOKE_DEPTH: usize = 10;
