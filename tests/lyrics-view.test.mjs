import {test} from 'node:test';
import assert from 'node:assert/strict';
import {displayParts,groupLines,activeLine,scrollTarget} from '../src/lib/utils/lyrics-view.ts';
test('verified thin-space bilingual separator preserves all content',()=>{const text='너 같은 남자는 No No No\u2009像你这种男人 不不不';assert.equal(displayParts(text).join('\u2009'),text);assert.equal(displayParts(text).length,2);assert.deepEqual(displayParts('한국어 中文 English 日本語'),['한국어 中文 English 日本語']);});
test('equal timestamps remain separate display lines in one active group',()=>{const rows=groupLines([{time_ms:100,text:'원문'},{time_ms:100,text:'翻译'},{time_ms:200,text:'next'}],true);assert.equal(rows.length,2);assert.deepEqual(rows[0].parts,['원문','翻译']);assert.equal(activeLine(rows,99),-1);assert.equal(activeLine(rows,100),0);assert.equal(activeLine(rows,200),1);assert.equal(activeLine(rows,100),0);});
test('plain lyrics never group at the sentinel timestamp',()=>assert.equal(groupLines([{time_ms:-1,text:'a'},{time_ms:-1,text:'b'}],false).length,2));
test('scroll target clamps to independent container bounds',()=>{assert.equal(scrollTarget(0,40,400,1200),0);assert.equal(scrollTarget(600,40,400,1200),460);assert.equal(scrollTarget(1190,40,400,1200),800);});
