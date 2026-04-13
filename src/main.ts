import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from "./router"; // 1. Import the router we just made

const app = createApp(App);
app.use(router); // 2. Tell Vue to use it
app.mount("#app");