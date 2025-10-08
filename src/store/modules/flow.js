const state = {
    // 当前正在编辑的节点
    currNode: {},
}


const mutations = {
    setCurrNode: (state, node) => {
        state.currNode = node
    },
    updateCurrNodeData: (state, data) => {
        state.currNode.data = data
    },
    updateCurrNodeOutput: (state, output) => {
        state.currNode.output = output
    },
    updateCurrNodeInput: (state, input) => {
        state.currNode.input = input
    }
}

const actions = {}

export default {
    namespaced: true,
    state,
    mutations,
    actions
}
