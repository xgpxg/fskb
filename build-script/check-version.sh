#!/bin/bash

# check-version.sh - 检查 VERSION 环境变量与配置文件中的版本是否一致

VERSION="$1"
# 检查 VERSION 环境变量是否设置
if [ -z "$VERSION" ]; then
    echo "错误: VERSION 未指定"
    exit 1
fi

echo "检查版本一致性..."
echo "本地构建版本: $VERSION"

# 获取 cargo.toml 中的版本
CARGO_VERSION=$(grep '^version = ' src-tauri/Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "Cargo.toml 版本: $CARGO_VERSION"

# 获取 tauri.conf.json 中的版本
TAURI_VERSION=$(grep '"version":' src-tauri/tauri.conf.json | head -1 | sed 's/.*"version": "\(.*\)".*/\1/')
echo "tauri.conf.json 版本: $TAURI_VERSION"

# 检查版本是否一致
if [ "$VERSION" != "$CARGO_VERSION" ]; then
    echo "错误: 指定版本 ($VERSION) 与 Cargo.toml 中的版本 ($CARGO_VERSION) 不一致"
    exit 1
fi

if [ "$VERSION" != "$TAURI_VERSION" ]; then
    echo "错误: 指定版本 ($VERSION) 与 tauri.conf.json 中的版本 ($TAURI_VERSION) 不一致"
    exit 1
fi

echo "版本检查通过: 所有版本一致 ($VERSION)"
exit 0
