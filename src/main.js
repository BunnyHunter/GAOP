import "./styles/global.scss";
import "./styles/select.css";

import "./vendors/element";
import "./vendors/fontawesome";
import "./vendors/vuerouter";
// import "./vendors/vue-katex";
import "./vendors/echarts";
import "./vendors/mona";

// install global api
import "./global_api";

import Vue from 'vue';

import Router from "./router/router";
import store from "./store/store";

import App from './App.vue';

Vue.config.productionTip = false
import ElementUI from 'element-ui'
import lang from 'element-ui/lib/locale/lang/en'
import locale from 'element-ui/lib/locale'
import 'element-theme-dark';
locale.use(lang)

Vue.filter("str", function (value) {
    return value.toString();
});

async function mount() {
    let monaApp = new Vue(
        {
            render: h => h(App),
            methods: {
                message(msg) {
                    this.$message(msg);
            },
        },
        router: Router,
        store,
    }).$mount('#app');

    window.monaApp = monaApp;
}

mount();

console.log(`Hello! Looking for a friend ?`);