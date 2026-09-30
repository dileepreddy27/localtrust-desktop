import http from 'node:http';
import {readFile} from 'node:fs/promises';
const files = {'/':'index.html','/index.html':'index.html','/style.css':'style.css','/app.js':'app.js','/demo.js':'demo.js'};
http.createServer(async (req,res) => {
  const file = files[req.url]; if (!file) {res.writeHead(404);res.end('Not found');return;}
  try {const body = await readFile(new URL('../ui/'+file,import.meta.url));res.writeHead(200,{'Content-Type':file.endsWith('.js')?'text/javascript':file.endsWith('.css')?'text/css':'text/html','X-Content-Type-Options':'nosniff'});res.end(body);} catch {res.writeHead(500);res.end('Unable to load demo');}
}).listen(4173,'127.0.0.1',()=>console.log('Synthetic demo: http://127.0.0.1:4173'));
