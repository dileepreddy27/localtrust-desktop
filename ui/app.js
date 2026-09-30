import { Demo } from './demo.js';
const demo = new Demo();
const native = Boolean(window.__TAURI__);
const invoke = native ? window.__TAURI__.core.invoke : demo.invoke.bind(demo);
const $ = id => document.getElementById(id);
let pending = null;
if (native) { $('mode').textContent = 'NATIVE · LOCAL'; $('notice').textContent = 'Real file operations · dedicated application workspace.'; }
function buttons(enabled) { $('approve').disabled = !enabled; $('deny').disabled = !enabled; }
async function preview(action) {
  if (pending) await invoke('decide', {id: pending.id, approve: false});
  pending = null; buttons(false);
  const p = await invoke('propose', {action}); pending = p;
  $('preview').textContent = JSON.stringify(p.action, null, 2);
  $('review-state').textContent = `Proposal #${p.id} · expires in 120 seconds`;
  buttons(true);
}
async function run(fn) { try { await fn(); } catch (e) { $('result').textContent = String(e); } finally {
  const events = await invoke('events'); $('events').replaceChildren(...events.slice(-8).map(e => { const li = document.createElement('li'); li.textContent = `Proposal #${e.id} · ${e.outcome}`; return li; }));
} }
$('propose').onclick = () => run(() => preview($('action').value === 'create' ? {action:'create',path:$('path').value,content:$('content').value} : {action:'read',path:$('path').value}));
$('plan').onclick = () => run(async () => { $('plan').disabled = true; try { await preview(await invoke('plan',{prompt:$('prompt').value,model:$('model').value})); } finally { $('plan').disabled = false; } });
for (const [id, approve] of [['approve',true],['deny',false]]) $(id).onclick = () => run(async () => {
  if (!pending) return; const p = pending; pending = null; buttons(false);
  $('review-state').textContent = `Proposal #${p.id} · consumed`;
  $('result').textContent = await invoke('decide',{id:p.id,approve});
});
$('action').onchange = () => { $('content').disabled = $('action').value === 'read'; };
