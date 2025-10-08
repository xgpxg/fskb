create table if not exists system_config
(
    config_key   text not null primary key,
    config_value text null
);

-- 知识库
create table if not exists knowledge_base
(
    id                        bigint               not null primary key, -- 主键
    name                      varchar(500)         not null,             -- 知识库名称
    description               text                 null,                 -- 知识库描述
    source                    tinyint              not null,             -- 知识库来源: 1平台 2自建
    table_name                varchar(500)         null,                 -- 对应的向量数据库表名
    icon                      text                 null,                 -- 图标
    nld                       text                 null,                 -- 知识库的自然语言描述（Natural language description）
    config                    text                 null,                 -- 配置,json格式
    mcp_server_ids            text                 null,                 -- MCP服务ID列表,数组格式
    model_id                  bigint               null,                 -- 语言模型ID
    file_content_extract_type tinyint(1)           null,                 -- 文件内容提取配置。该配置针对整个知识库生效，在knowledge_base_import_record中会冗余一份，记录其使用的是哪种抽取方式
    create_user_id            bigint               null,                 -- 创建人id
    update_user_id            bigint               null,                 -- 修改人ID
    create_time               datetime             null,                 -- 创建时间
    update_time               datetime             null,                 -- 更新时间
    remark                    varchar(500)         null,                 -- 备注
    user_id                   bigint               null,                 -- 用户id
    is_delete                 tinyint(1) default 0 null                  -- 是否删除
);

-- 聊天消息
create table if not exists chat_message
(
    id                bigint               not null primary key, -- 主键
    message_id        bigint               not null,             -- 消息递增ID
    parent_message_id bigint               not null,             -- 父级消息ID
    knowledge_base_id bigint               not null,             -- 知识库ID
    role              varchar(20)          null,                 -- 消息角色：system、user、assistant
    status            varchar(20)          null,                 -- 消息状态：pending | success | fail
    content           text                 not null,             -- 消息内容
    create_user_id    bigint               null,                 -- 创建人ID
    update_user_id    bigint               null,                 -- 修改人ID
    create_time       datetime             null,                 -- 创建时间
    update_time       datetime             null,                 -- 更新时间
    remark            varchar(500)         null,                 -- 备注
    user_id           bigint               null,                 -- 用户ID
    tenant_id         bigint               null,                 -- 租户ID
    is_delete         tinyint(1) default 0 null                  -- 是否删除
);

-- 知识库导入记录
create table if not exists knowledge_base_import_record
(
    id                        bigint               not null primary key, -- 主键
    knowledge_base_id         bigint               null,                 -- 知识库ID
    source                    tinyint(1)           null,                 -- 来源：1本地文件 2网页 3自定义文本
    title                     text                 null,                 -- 标题
    original_file_name        text                 null,                 -- 原始文件名称
    original_file_path        text                 null,                 -- 原始文件路径
    file_name                 text                 null,                 -- 文件名称
    file_size                 int                  null,                 -- 文件大小
    file_path                 text                 null,                 -- 文件路径
    file_content_type         tinyint(1)           null,                 -- 文件内容类型：1文档 2数据表
    file_content_extract_type text                 null,                 -- 文件内容提取配置
    url                       text                 null,                 -- 网页地址
    nld                       text                 null,                 -- 导入记录的自然语言描述（Natural language description）
    status                    tinyint(1)           null,                 -- 状态：0待解析 1导入成功 2导入中 3导入失败
    status_msg                text                 null,                 -- 状态信息
    start_time                datetime             null,                 -- 开始时间
    end_time                  datetime             null,                 -- 结束时间
    create_user_id            bigint               null,                 -- 创建人ID
    update_user_id            bigint               null,                 -- 修改人ID
    create_time               datetime             null,                 -- 创建时间
    update_time               datetime             null,                 -- 更新时间
    remark                    varchar(500)         null,                 -- 备注
    user_id                   bigint               null,                 -- 用户ID
    is_delete                 tinyint(1) default 0 null                  -- 是否删除
);

-- 模型定义
create table if not exists model
(
    id             bigint               not null primary key, -- 主键
    name           text                 not null,             -- 模型名称，全局唯一
    description    text                 null,                 -- 模型描述
    source         tinyint              not null,             -- 模型来源：1：内置 2：自建
    icon           text                 null,                 -- 模型图标
    status         tinyint              not null,             -- 状态：0未启用 1已启用 2异常 3安装中（仅离线模型） 4启动中（仅离线模型）
    status_msg     text                 null,                 -- 状态消息
    base_url       text                 not null,             -- 请求地址
    api_key        text                 null,                 -- api key
    max_token      bigint               null,                 -- 最大token
    task_type      tinyint(1)           null,                 -- 适用的任务类型：1文本生成 2视觉问答
    create_user_id bigint               null,                 -- 创建人id
    update_user_id bigint               null,                 -- 修改人ID
    create_time    datetime             null,                 -- 创建时间
    update_time    datetime             null,                 -- 更新时间
    remark         varchar(500)         null,                 -- 备注
    user_id        bigint               null,                 -- 用户id
    is_delete      tinyint(1) default 0 null                  -- 是否删除
);


create table if not exists mcp_server
(
    id                bigint       not null primary key,   -- 主键
    name              text         not null,               -- 服务名称，全局唯一
    summary           text         not null,               -- 服务简介
    description       text         null,                   -- 描述
    config            text         not null,               -- 配置，json格式
    source            tinyint      not null,               -- 来源：1平台内置 2自定义
    configurable      tinyint      not null default 0,     -- 是否可配置
    status            tinyint      not null default 0,     -- 状态：0未启用 1已启用 2异常 3安装中 4启动中 5停止中 6升级中
    status_msg        text         null,                   -- 状态信息
    installed_version text         null,                   -- 已安装版本
    latest_version    text         null,                   -- 最新版本
    create_user_id    bigint       null,                   -- 创建人id
    update_user_id    bigint       null,                   -- 修改人ID
    create_time       datetime     null,                   -- 创建时间
    update_time       datetime     null,                   -- 更新时间
    remark            varchar(500) null,                   -- 备注
    user_id           bigint       null,                   -- 用户id
    is_delete         tinyint(1)            default 0 null -- 是否删除
);

create table if not exists mcp_server_define
(
    id             bigint       not null primary key,   -- 主键
    name           text         not null,               -- 服务名称，全局唯一
    summary        text         not null,               -- 服务简介
    description    text         null,                   -- 描述
    config         text         not null,               -- 配置，json格式
    url            text         null,                   -- 下载地址
    type           text         null,                   -- stdio | sse
    configurable   tinyint      not null default 0,     -- 是否可配置
    version        text         null,                   -- 版本
    create_user_id bigint       null,                   -- 创建人id
    update_user_id bigint       null,                   -- 修改人ID
    create_time    datetime     null,                   -- 创建时间
    update_time    datetime     null,                   -- 更新时间
    remark         varchar(500) null,                   -- 备注
    user_id        bigint       null,                   -- 用户id
    is_delete      tinyint(1)            default 0 null -- 是否删除
);

create table if not exists user_profile
(
    id                      bigint               not null primary key,
    enable_profile_memory   tinyint(1) default 0 null, -- 是否启用画像记忆,0: 否, 1: 是
    profile_memory_model_id bigint               null, -- 记忆提取使用的模型ID
    create_user_id          bigint               null, -- 创建人id
    update_user_id          bigint               null, -- 修改人ID
    create_time             datetime             null, -- 创建时间
    update_time             datetime             null, -- 更新时间
    remark                  varchar(500)         null, -- 备注
    user_id                 bigint               null, -- 用户id
    is_delete               tinyint(1) default 0 null  -- 是否删除
);

-- 笔记
create table if not exists note
(
    id                bigint               not null primary key, -- 主键
    knowledge_base_id bigint               not null,             -- 知识库ID
    title             text                 null,                 -- 标题
    summary           text                 null,                 -- 摘要
    content           text                 null,                 -- 内容
    create_user_id    bigint               null,                 -- 创建人id
    update_user_id    bigint               null,                 -- 修改人ID
    create_time       datetime             null,                 -- 创建时间
    update_time       datetime             null,                 -- 更新时间
    remark            varchar(500)         null,                 -- 备注
    user_id           bigint               null,                 -- 用户id
    is_delete         tinyint(1) default 0 null                  -- 是否删除
);

-- insert or ignore into knowledge_base(id, name, description, icon, source, create_time)
-- values (0, '小飞树', '小飞树', '/images/xfs.png', 1, datetime());


---------------------------------内置模型-----------------------------
-- deepseek
insert or ignore into model (id, name, description, source, icon, status, base_url, task_type, create_time)
values (1, 'deepseek-chat', 'Deepseek-V3版本', 1,
        '/images/model-icon/model-deepseek.png', 0,
        'https://api.deepseek.com', 1,
        datetime());
-- 通义千问
insert or ignore into model (id, name, description, source, icon, status, base_url, task_type, create_time)
values (2, 'qwen-plus', '通义千问文本模型', 1,
        '/images/model-icon/model-qwen.png', 0,
        'https://dashscope.aliyuncs.com/compatible-mode/v1', 1,
        datetime());
-- 豆包
insert or ignore into model (id, name, description, source, icon, status, base_url, task_type, create_time)
values (3, 'doubao-1-5-pro-32k-250115', '豆包文本模型', 1,
        '/images/model-icon/model-doubao.png', 0,
        'https://ark.cn-beijing.volces.com/api/v3', 1,
        datetime());
-- 智谱AI
insert or ignore into model (id, name, description, source, icon, status, base_url, task_type, create_time)
values (4, 'glm-4-flash-250414', '智谱AI文本模型(免费版)', 1,
        '/images/model-icon/model-zhipuqingyan.png', 0,
        'https://open.bigmodel.cn/api/paas/v4', 1,
        datetime());
-- 腾云混元
insert or ignore into model (id, name, description, source, icon, status, base_url, task_type, create_time)
values (5, 'hunyuan-turbos-latest', '腾讯混元文本模型', 1,
        '/images/model-icon/model-hunyuan.png', 0,
        'https://api.hunyuan.cloud.tencent.com/v1', 1,
        datetime());
-- 智谱多模态模型
insert or ignore into model (id, name, description, source, icon, status, base_url, task_type, create_time)
values (6, 'glm-4v-flash', '智谱AI视觉问答模型(免费版)', 1,
        '/images/model-icon/model-zhipuqingyan.png', 0,
        'https://open.bigmodel.cn/api/paas/v4', 2,
        datetime());
insert or ignore into model (id, name, description, source, icon, status, base_url, task_type, create_time)
values (7, 'glm-4.5', '智谱AI文本模型', 1,
        '/images/model-icon/model-zhipuqingyan.png', 0,
        'https://open.bigmodel.cn/api/paas/v4', 1,
        datetime());
-------------------------------------------------------------------------

---------------------------------个性化设置-----------------------------
insert or ignore into user_profile(id, enable_profile_memory, profile_memory_model_id, create_time)
values (1, 0, null, datetime());
