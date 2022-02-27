import {EditorState,Compartment} from 'https://jspm.dev/@codemirror/state';
import {EditorView,keymap,dropCursor} from 'https://jspm.dev/@codemirror/view';
import {lineNumbers,highlightActiveLineGutter} from 'https://jspm.dev/@codemirror/gutter';
import {foldGutter,foldKeymap} from 'https://jspm.dev/@codemirror/fold';
import {lintGutter} from 'https://jspm.dev/@codemirror/lint';
import {bracketMatching} from 'https://jspm.dev/@codemirror/matchbrackets';
import {closeBrackets} from 'https://jspm.dev/@codemirror/closebrackets';
import {indentWithTab,standardKeymap} from 'https://jspm.dev/@codemirror/commands';
import {indentOnInput} from 'https://jspm.dev/@codemirror/language';
import {json} from 'https://jspm.dev/@codemirror/lang-json';
// import {oneDark} from "https://jspm.dev/@codemirror/theme-one-dark";
import {materialDark} from "https://jspm.dev/@ddietr/codemirror-themes/theme/material-dark"

const doc='{}';
const tabSize=new Compartment();
const lineWrapping=new Compartment();
const bracketClosing=new Compartment();
const baseTheme=EditorView.baseTheme(
  {
    '.cm-content, .cm-gutter':{minHeight:'100vh'},
    '.cm-scroller':{overflow:'auto'}
  }
);
const extensions=[
  keymap.of([...standardKeymap,...foldKeymap,indentWithTab]),
  materialDark,
  baseTheme,
  tabSize.of(EditorState.tabSize.of(2)),
  lineWrapping.of(EditorView.lineWrapping),
  dropCursor(),
  lineNumbers(),
  highlightActiveLineGutter(),
  foldGutter(),
  lintGutter(),
  bracketMatching(),
  bracketClosing.of(closeBrackets()),
  indentOnInput(),
  json(),
];
const state=EditorState.create({doc,extensions});
const installOn=parent=>{
  return new EditorView({state,parent});
};
export default installOn;
