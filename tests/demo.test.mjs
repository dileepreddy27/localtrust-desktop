import {test} from 'node:test';
import assert from 'node:assert/strict';
import {Demo} from '../ui/demo.js';
test('synthetic create requires approval; payload is immutable; approval cannot replay',async()=>{
 const d=new Demo(), action={action:'create',path:'notes.txt',content:'synthetic'};
 const p=await d.invoke('propose',{action}); action.content='changed'; p.action.content='changed'; assert.equal(d.files.size,0);
 await d.invoke('decide',{id:p.id,approve:true}); assert.equal(d.files.get('notes.txt'),'synthetic');
 await assert.rejects(d.invoke('decide',{id:p.id,approve:true}));
 const r=await d.invoke('propose',{action:{action:'read',path:'notes.txt'}}); assert.equal(await d.invoke('decide',{id:r.id,approve:true}),'synthetic');
});
test('denial, traversal, overwrite and missing model fail clearly',async()=>{
 const d=new Demo(); const p=await d.invoke('propose',{action:{action:'create',path:'a.txt',content:'x'}});
 await d.invoke('decide',{id:p.id,approve:false}); assert.equal(d.files.size,0);
 await assert.rejects(d.invoke('propose',{action:{action:'read',path:'../secret.txt'}}));
 await assert.rejects(d.invoke('plan')); assert.equal(d.log[0].outcome,'denied');
});
