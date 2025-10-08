import {convertImageSrc} from "../../utils/commands.ts";
import {sep} from '@tauri-apps/api/path';

class UserMessageContent {
    text: string;
    images: string[];
    files: string[];
    command: string;

    constructor(text: string = '', images: string[] = [], files: string[] = []) {
        this.text = text
        this.images = images
        this.files = files
    }

    static from(value: string | object): UserMessageContent {
        let obj: {
            images?: string[];
            text?: string;
            files?: string[];
        };
        if (typeof value === 'string') {
            obj = JSON.parse(value)
        } else {
            obj = value;
        }
        const text = obj.text;
        const images = obj.images;
        const files = obj.files;
        return new UserMessageContent(text, images, files);
    }

    toMd() {
        let content = this.text
        if (this.images && this.images.length > 0) {
            content += this.images.map(image => `![image](${convertImageSrc(image)})`).join('\n')
        }
        if (this.files && this.files.length > 0) {
            content += this.files.map(file => renderFile(file)).join('\n')
        }
        return content
    }

    isEmpty(): boolean {
        return !this.text.trim() && this.images.length === 0 && this.files.length === 0
    }
}

import {openPath} from "@tauri-apps/plugin-opener";

// 渲染消息框中的文件卡片
function renderFile(file: string): string {
    // 提取文件名
    const fileName = file.split(sep()).pop();
    const fileExtension = fileName.split('.').pop()?.toLowerCase() || '';

    return `
        <div class="file-card" 
            data-filepath="${file}"
            data-ext="${fileExtension}">
            <div class="file-ext-card">
                ${fileExtension.substring(0, 3).toUpperCase()}
            </div>
            ${fileName}
        </div>
    `.replace(/\s+/g, ' ').trim();
}

const addFileCardClickEvent = () => {
    // 检查是否已经添加过监听器
    if ((window as any).__FILE_CLICK_HANDLER_ADDED) {
        return;
    }
    const handleFileClick = (event: Event) => {
        const target = event.target as HTMLElement;
        if (target.dataset && target.dataset['filepath']) {
            const filePath = target.dataset['filepath'];
            if (filePath) {
                openPath(filePath).then();
            }
        }
    };
    document.addEventListener('click', handleFileClick);

    (window as any).__FILE_CLICK_HANDLER_ADDED = true;
};

class AssistantMessageContent {
    text: string;


    constructor(text: string) {
        this.text = text
    }

    static from(value: string): AssistantMessageContent {
        // 提取markdown中的图片，并在图片地址前加上http://asset.localhost/
        const imgRegex = /!\[([^\]]*)]\(([^)]+)\)/g;
        let processedValue = value.replace(imgRegex, (match, alt, src) => {
            // 如果图片地址已经是完整URL，则不处理
            if (src.startsWith('http://') || src.startsWith('https://')) {
                return match;
            }
            // 如果图片地址是相对路径，则加上前缀
            const fullUrl = `http://asset.localhost/${src.startsWith('/') ? src.substring(1) : src}`;
            return `![${alt}](${fullUrl})`;
        });
        processedValue = AssistantMessageContent.preprocessMath(processedValue)
        return new AssistantMessageContent(processedValue);
    }

    toMd() {
        return this.text
    }

    // 解决公式$之间空格问题
    static preprocessMath(content: string) {
        // 使用正则表达式去除行内公式首尾空格
        return content.replace(/\$(\s*)(.*?)(\s*)\$/g, (match, leadingSpace, formula, trailingSpace) => {
            return `$${formula.trim()}$`;
        });
    }
}

addFileCardClickEvent()

export {
    UserMessageContent,
    AssistantMessageContent
}