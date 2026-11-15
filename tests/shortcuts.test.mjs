import {test} from 'node:test';
import assert from 'node:assert/strict';
import {shortcut} from '../src/lib/utils/shortcuts.ts';
const key=(value,overrides={})=>({key:value,ctrlKey:false,altKey:false,metaKey:false,shiftKey:false,repeat:false,isComposing:false,defaultPrevented:false,...overrides});
test('window actions map to requested keys',()=>{
 for(const [value,action] of [['f','search'],['ArrowLeft','previous'],['ArrowRight','next'],['l','lyrics'],['o','open']])assert.equal(shortcut(key(value,{ctrlKey:true}),false,false,false),action);
 assert.equal(shortcut(key(' '),false,false,false),'toggle');
 assert.equal(shortcut(key('Escape'),true,false,true),'escape');
});
test('editing, native controls and dialogs preserve their keyboard behavior',()=>{
 assert.equal(shortcut(key(' '),true,false,false),null);
 assert.equal(shortcut(key(' '),false,true,false),null);
 for(const value of ['ArrowLeft','ArrowRight','l','o'])assert.equal(shortcut(key(value,{ctrlKey:true}),true,false,false),null);
 assert.equal(shortcut(key('f',{ctrlKey:true}),true,false,false),'search');
 assert.equal(shortcut(key(' '),false,false,true),null);
});
test('repeat, composition, handled events and extra modifiers do not dispatch',()=>{
 for(const flag of ['repeat','isComposing','defaultPrevented','altKey','metaKey','shiftKey'])assert.equal(shortcut(key(' ',{[flag]:true}),false,false,false),null);
 assert.equal(shortcut(key('k',{ctrlKey:true}),false,false,false),null);
});
