#!/bin/bash

# check-resources-version.sh - 检查 VERSION 变量与 resources/.version 文件中的版本是否一致

# 检查 VERSION 环境变量是否设置
if [ -z "$VERSION" ]; then
    echo "错误: VERSION 未指定"
    exit 1
fi

echo "检查 resources/.version 文件中的版本一致性..."
echo "本次构建版本: $VERSION"

# 检查 resources/.version 文件是否存在
if [ ! -f "src-tauri/resources/.version" ]; then
    echo "错误: src-tauri/resources/.version 文件不存在"
    exit 1
fi

# 获取 resources/.version 文件中的版本号
RESOURCES_VERSION=$(cat src-tauri/resources/.version)
echo "resources/.version 文件中的版本: $RESOURCES_VERSION"

# 检查版本是否一致
if [ "$VERSION" != "$RESOURCES_VERSION" ]; then
    echo "错误: 指定版本 ($VERSION) 与 resources/.version 文件中的版本 ($RESOURCES_VERSION) 不一致"
    exit 1
fi

echo "resources 版本检查通过: 版本一致 ($VERSION)"
exit 0
