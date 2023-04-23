import { h, Component, render } from "https://esm.sh/preact";

document.addEventListener("DOMContentLoaded", () => {
  let counter = 0;

  async function fetchCpuData() {
    try {
      const response = await fetch("/api/cpu");
      if (response.status !== 200) {
        throw new Error(`HTTP error! satus: ${response.status}`);
      }

      const jsonData = await response.json();
      counter++;

      const app = h("pre", null, JSON.stringify(jsonData, null, 2));
      render(app, document.body);
    } catch (error) {
      console.error("Error fetching CPU data: ", error);
    }
  }

  setInterval(fetchCpuData, 1000);
});
