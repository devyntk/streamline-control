import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

import "./styles/index.scss";

// @ts-ignore
globalThis.__VUE_OPTIONS_API__ = process.env.NODE_ENV === "development";
// @ts-ignore
globalThis.__VUE_PROD_DEVTOOLS__ = process.env.NODE_ENV === "development";

const app = createApp(App);

app.use(router);

app.mount("#app");
