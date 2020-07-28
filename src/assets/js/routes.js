import Home from "./components/Home";

const routes = [
  { path: '/', name: 'home', component: Home},
  { path: "*", redirect: { name: 'home' }}
];

export default routes;
