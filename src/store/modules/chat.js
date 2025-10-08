const state = {
    replying: false,
}


const mutations = {
    setReplying: (state, replying) => {
        state.replying = replying
    },

}

const actions = {}

export default {
    namespaced: true,
    state,
    mutations,
    actions
}
