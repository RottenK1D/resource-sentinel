import { h, render } from "https://esm.sh/preact";
import htm from "https://unpkg.com/htm?module";

// Initialize htm with Preact
const html = htm.bind(h);

function App(props) {
  return html`
    <h1>LIVE CPU MONITOR</h1>
    <div>
      ${props.cpus.map((cpu, i) => {
        i++;
        return html`
          <div class="outer">
            <div class="cores">CPU ${i}</div>
            <div class="cpu">${cpu.toFixed(1)} %</div>
          </div>
        `;
      })}
    </div>
  `;
}

let url = new URL("/realtime/cpu", window.location.href);

url.protocol = url.protocol.replace("http", "ws");

let ws = new WebSocket(url.href);
ws.onmessage = (ev) => {
  let json = JSON.parse(ev.data);
  render(html`<${App} cpus=${json}></${App}>`, document.body);
};
