import {convertFileSrc, invoke} from "@tauri-apps/api/core";
import {ElMessage} from "element-plus";

const IS_DEV = import.meta.env.VITE_IS_DEV

const isDev = () => {
    return IS_DEV === 1 || IS_DEV === '1'
}

type Res = {
    code: number,
    msg: string,
    data: any
}
const call = async (command: string, args: any, options = {showError: true}) => {
    try {
        const res: Res = await invoke(command, args)
        if (res.code !== 0) {
            let msg = res.msg
            if (isDev()) {
                msg += '，参数：'
                msg += JSON.stringify(args)
            }
            throw new Error(msg)
        }
        return res.data
    } catch (e) {
        if (options.showError === true) {
            ElMessage.error({
                message: e.message || e,
                plain: true
            })
        }
        throw e
    }

}

const convertImageSrc = (path: string) => {
    return convertFileSrc(path)
}

export {
    call,
    convertImageSrc
}