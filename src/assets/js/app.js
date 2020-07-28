import Vue from 'vue';
import VueRouter from 'vue-router';
import routes from './routes';

Vue.use(VueRouter);

Vue.config.productionTip = true;

new Vue({
  el: '#app',
  router: new VueRouter({ routes, mode: 'history' }),
});
