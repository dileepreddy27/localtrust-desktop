import {chromium} from 'playwright';
import assert from 'node:assert/strict';
const browser = await chromium.launch({headless:true,executablePath:process.env.CHROMIUM_EXECUTABLE});
const page=await browser.newPage({viewport:{width:1440,height:1100},deviceScaleFactor:1});
const errors=[];page.on('pageerror',e=>errors.push(e.message));
await page.goto('http://127.0.0.1:4173');
await page.getByRole('button',{name:'Preview action'}).click();
await page.getByRole('button',{name:'Deny',exact:true}).click();
assert.match(await page.locator('#result').innerText(),/Denied/);
await page.getByRole('button',{name:'Preview action'}).click();
await page.getByRole('button',{name:'Approve once'}).click();
assert.match(await page.locator('#result').innerText(),/Created synthetic/);
await page.locator('#action').selectOption('read');
await page.getByRole('button',{name:'Preview action'}).click();
await page.getByRole('button',{name:'Approve once'}).click();
assert.match(await page.locator('#result').innerText(),/Synthetic design review/);
await page.getByRole('button',{name:'Preview action'}).click();
await page.screenshot({path:'docs/demo.png',fullPage:true});
await page.locator('#path').fill('../private.txt');
await page.getByRole('button',{name:'Preview action'}).click();
assert.match(await page.locator('#result').innerText(),/simple .txt/);
await page.setViewportSize({width:390,height:844});
assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth<=window.innerWidth),true);
assert.deepEqual(errors,[]);
console.log('PASS: deny, approve, read-back, traversal rejection, mobile overflow, no browser errors.');
await browser.close();


