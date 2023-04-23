document.addEventListener("DOMContentLoaded", () => {
  let i = 0;

  setInterval(() => {
    i++;
    document.body.textContent = `cycle ${i}`;
  }, 1000);
});
