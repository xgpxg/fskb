const state = {
    currMcpServer: {},
}


const mutations = {
    setCurrMcpServer: (state, server) => {
        state.currMcpServer = server
    },
}

const actions = {}

export default {
    namespaced: true,
    state,
    mutations,
    actions
}
