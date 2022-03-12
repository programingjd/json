import {EditorState,Compartment} from 'https://jspm.dev/@codemirror/state';
import {EditorView,keymap} from 'https://jspm.dev/@codemirror/view';
import {history} from 'https://jspm.dev/@codemirror/history';
import {lineNumbers,highlightActiveLineGutter} from 'https://jspm.dev/@codemirror/gutter';
import {foldGutter,foldKeymap,codeFolding} from 'https://jspm.dev/@codemirror/fold';
import {search,searchKeymap} from 'https://jspm.dev/@codemirror/search';
import {lintGutter} from 'https://jspm.dev/@codemirror/lint';
import {bracketMatching} from 'https://jspm.dev/@codemirror/matchbrackets';
import {closeBrackets} from 'https://jspm.dev/@codemirror/closebrackets';
import {indentWithTab,standardKeymap} from 'https://jspm.dev/@codemirror/commands';
import {indentOnInput} from 'https://jspm.dev/@codemirror/language';
import {json} from 'https://jspm.dev/@codemirror/lang-json';
import {darcula} from "./darcula.js";

const doc=JSON.stringify({"key":"value","array":[1,true,null,"text"],"object":{}},null,2);
const tabSize=new Compartment();
const lineWrapping=new Compartment();
const bracketClosing=new Compartment();
const baseTheme=EditorView.baseTheme(
  {
    '.cm-content, .cm-gutter':{minHeight:'100vh',fontFamily:'ccpl,serif'},
    '.cm-scroller':{overflow:'auto'},
    '.cm-activeLineGutter':{fontWeight:'bold'},
  }
);
const extensions=[
  keymap.of([...standardKeymap,...foldKeymap,...searchKeymap,indentWithTab]),
  history(),
  darcula,
  baseTheme,
  tabSize.of(EditorState.tabSize.of(2)),
  lineWrapping.of(EditorView.lineWrapping),
  lineNumbers(),
  highlightActiveLineGutter(),
  codeFolding({placeholderText:'...'}),
  foldGutter({openText:'-',closedText:'+'}),
  lintGutter(),
  bracketMatching(),
  bracketClosing.of(closeBrackets()),
  search({top:true}),
  indentOnInput(),
  json(),
];
const state=EditorState.create({doc,extensions});
/** @return {EditorView} */
const installOn=parent=>{
  return new EditorView({state,parent});
};
export default installOn;
