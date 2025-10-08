#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

RESOURCES_NAME="resources.tar.gz"
VERSION="$1"
if [ -z "${VERSION}" ]; then
    echo "请指定版本号"
    read -n 1 -s -r
    exit 1
fi

echo "开始打包${RESOURCES_NAME}，版本：${VERSION}"

# 检查版本一致性
"${SCRIPT_DIR}/check-resources-version.sh" ${VERSION}
if [ $? -ne 0 ]; then
    echo "版本检查失败，打包终止"
    read -n 1 -s -r
    exit 1
fi

tar -czvf package/${RESOURCES_NAME} -C src-tauri/resources .
echo "打包完成！"