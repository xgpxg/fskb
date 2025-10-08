#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

APP_NAME="fs-kb-app"
VERSION="$1"
if [ -z "${VERSION}" ]; then
    echo "请指定版本号"
    read -n 1 -s -r
    exit 1
fi

echo "开始打包${APP_NAME}，版本：${VERSION}"

# 检查版本一致性
"${SCRIPT_DIR}/check-version.sh" ${VERSION}
if [ $? -ne 0 ]; then
    echo "版本检查失败，打包终止"
    read -n 1 -s -r
    exit 1
fi

# 执行打包
npm run tauri build
echo "打包完成！"