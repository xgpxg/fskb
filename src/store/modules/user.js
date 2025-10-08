const state = {
    // 登录状态
    isLogin: false,
    // token
    token: '',
    // 昵称
    nickname: '',
    // 头像
    avatar: '',
    // 脱敏手机号
    phone: '',
    // 脱敏邮箱
    email: '',
    // 角色
    roles: '',
    // 权限
    permissions: [],
    // 会员信息
    member: {
        // 过期时间
        expTime: ''
    },
    other: {
        // 密码是否设置
        passwordIsSet: false,
    }
}

//初始化
const initState = (state, token) => {
    if (token) {
        state.token = token
        state.isLogin = true
    }
}

initState(state, localStorage.getItem('token'))


const mutations = {
    setToken: (state, token) => {
        state.token = token
        state.isLogin = true
        localStorage.setItem('token', token)
    },
    setNickname: (state, nickname) => {
        state.nickname = nickname
    },
    setAvatar: (state, avatar) => {
        state.avatar = avatar
    },
    setPhone: (state, phone) => {
        state.phone = phone
    },
    setEmail: (state, email) => {
        state.email = email
    },
    setMember: (state, member) => {
        state.member = member
    },
    setOther: (state, other) => {
        state.other = other
    },
    setRole: (state, role) => {
        state.roles = role
    },
    setPermissions: (state, permissions) => {
        state.permissions = permissions
    },
    resetToken: (state) => {
        state.isLogin = false
        state.token = ''
        state.nickname = ''
        state.avatar = ''
        state.member = {}
        localStorage.removeItem('token')
        localStorage.removeItem('mrag:mobile')
    },
}

const actions = {}

export default {
    namespaced: true,
    state,
    mutations,
    actions
}
