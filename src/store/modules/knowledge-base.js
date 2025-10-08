const state = {
    list: [],
}


const mutations = {
    setList: (state, list) => {
        state.list = list
    },
}

const actions = {}

export default {
    namespaced: true,
    state,
    mutations,
    actions
}
