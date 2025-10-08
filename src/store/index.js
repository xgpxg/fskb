import {createStore} from 'vuex'
import getters from './getters'
import user from './modules/user'
import chat from "./modules/chat";
import knowledgeBase from "./modules/knowledge-base";
import mcp from "./modules/mcp";
import flow from "./modules/flow";

const store = createStore({
    modules: {
        user,
        chat,
        knowledgeBase,
        mcp,
        flow
    },
    getters
})


export default store
