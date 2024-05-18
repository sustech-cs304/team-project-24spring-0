import Editor, {useMonaco} from "@monaco-editor/react";
import React, {useEffect, useRef} from "react";
import Image from "next/image";
import { invoke } from '@tauri-apps/api/tauri';
import useOutputStore from "@/utils/outputState";
import useFileStore from "@/utils/state";
import rv32i from "@/constants/riscv/rv32i.json"
const language_id = 'riscv';

function getDifference(a, b)
{
    var i = 0;
    var j = 0;
    var result = "";

    while (j < b.length)
    {
     if (a[i] != b[j] || i == a.length)
         result += b[j];
     else
         i++;
     j++;
    }
    return result;
}


export default function ModifiedEditor({fileName}) {
    const monacoRef = useRef(null);
    const editorRef = useRef(null);
    const state = useFileStore();
    const file = useFileStore(state => state.files.find(file => file.fileName === fileName));

    function handleEditorDidMount(editor, monaco) {
        // here is the editor instance
        // you can store it in `useRef` for further usage
        editorRef.current = editor;
        monacoRef.current = monaco;
    }
    

    var handleEditorChange = async (value) => {
        let newInput = getDifference(file.code, value);
        state.updateFile(fileName, value, file.original, file.assembly, file.runLines);
        let position = editorRef.current.getPosition();
        let line = position.lineNumber;
        let column = position.column;
        console.log('Current: line: ', line, 'column: ', column, 'value: ', newInput);
    }

    var handleClickedRun = async () => {
        var line = editorRef.current.getPosition();
        var range = new monaco.Range(line.lineNumber, 1, line.lineNumber, 1);
        var id = { major: 1, minor: 1 };             
        var text = "FOO";
        var op = {identifier: id, range: range, text: text, forceMoveMarkers: false};
        editorRef.current.executeEdits("my-source", [op]);
    }

    return (
        <div className='h-full relative'>
            <Editor 
            theme={language_id}
            language={language_id} 
            className='overflow-hidden h-full'
            value={file.code}
            onChange={handleEditorChange}
            onMount={handleEditorDidMount}
            options={
                { hover: { enabled: true } }
            }
            beforeMount={LoadMonacoConfig}
            />
            <div className='absolute right-2 top-0 flex-row gap-2'>
                <button className='bg-gray-100 rounded-2xl hover:bg-gray-200' onClick={handleClickedRun}>
                    <Image alt="run icon" src='/icons/run.svg' width={16} height={16}/>
                </button>
            </div>
        </div>

    );
}

function LoadMonacoConfig(monaco) {
    monaco.languages.register({ id: language_id });

    monaco.languages.setMonarchTokensProvider(language_id, getRiscvMonarchTokensProvider());

    monaco.languages.registerCompletionItemProvider(language_id, getRiscvCompletionProvider());

    monaco.languages.registerHoverProvider(language_id, getRiscvHoverProvider());

    monaco.editor.defineTheme(language_id, {
        base: 'vs-dark',
        inherit: true,
        rules: [
            { token: 'comment', foreground: '529456' },
            { token: 'number', foreground: 'B5CEA8' },
            { token: 'string', foreground: 'CB926B' },
            { token: 'register', foreground: '92DBFD' },
            { token: 'operator', foreground: '41C9B0' },
            { token: 'label', foreground: '5BACE4' },
            { token: 'directive', foreground: 'C485BF' },
            { token: 'unknown', foreground: 'EC4D4E' }
        ],
        colors: {
            'editor.foreground': '#F8F8F2',
            'editor.background': '#272822',
            'editor.selectionBackground': '#49483E',
            'editor.lineHighlightBackground': '#3E3D32',
            'editorCursor.foreground': '#F8F8F0',
            'editorWhitespace.foreground': '#3B3A32',
            'editorIndentGuide.background': '#3B3A32',
            'editorLineNumber.foreground': '#75715E',
            'editorLineNumber.activeForeground': '#F8F8F0',
            'editorCursor.background': '#A7A7A7'
        }
    });
}

function getRiscvMonarchTokensProvider() {
    let directive = rv32i.directive;
    return {
        seperator: /[,:\s]/,

        register: Object.keys(rv32i.register),
        operator: Object.keys(rv32i.operator),

        tokenizer: {
            root: [
                [/#.*$/, 'comment'],
                [/(0[xX][0-9a-fA-F]+|\d+)(?=@seperator|$)/, 'number'],
                [/"(?:[^\\"]*(?:\\.)*)*"/, 'string'],
                [/[a-zA-Z_][\w.]*(?=@seperator|$)/, {
                    cases: {
                        '@register': 'register',
                        '@operator': 'operator',
                        '@default': 'label'
                    }
                }],
                [new RegExp(`(${directive.join('|').replace(/\./g, '\\.')})(?=@seperator|$)`), 'directive'],
                [/[^,:\s][\.\w]*(?=\W|$)/, 'unknown']
            ],
        }
    };
}

function getRiscvCompletionProvider() {
    let directive = rv32i.directive;
    let register_map = rv32i.register;
    let register_key = Object.keys(register_map);
    let operator_map = rv32i.operator;
    let operator_key = Object.keys(operator_map);

    let directive_items = directive.map(directive => {
        return {
            label: directive,
            kind: monaco.languages.CompletionItemKind.Keyword,
            detail: directive,
            sortText: '2' + directive,
            range: null,
            insertText: directive
        }
    });

    let register_items = register_key.map((register, idx) => {
        return {
            label: register,
            kind: monaco.languages.CompletionItemKind.Value,
            detail: `Register ${register_map[register]}`,
            sortText: '1' + String(idx).padStart(2, '0'),
            range: null,
            insertText: register
        }
    });

    let operator_items = [];
    for (let operator of operator_key) {
        if (operator_map[operator].length === 0) {
            operator_items.push({
                label: operator,
                kind: monaco.languages.CompletionItemKind.Function,
                detail: 'unimplemented',
                sortText: '3' + operator,
                range: null,
                insertText: operator
            });
        } else {
            for (let hint of operator_map[operator]) {
                operator_items.push({
                    label: operator,
                    kind: monaco.languages.CompletionItemKind.Function,
                    detail: hint,
                    sortText: '3' + operator,
                    range: null,
                    insertText: operator
                });
            }
        }
    }

    let all_items = directive_items.concat(register_items).concat(operator_items);
    let items_without_operator = directive_items.concat(register_items);

    return {
        triggerCharacters: [
            ...'abcdefghijklmnopqrstuvwxyz',
            ...'ABCDEFGHIJKLMNOPQRSTUVWXYZ',
            ...'0123456789',
            '_', '.'
        ],

        provideCompletionItems: (model, position, context, token) => {
            let find = model.findPreviousMatch(/[\w\.]*/, position, true, true, null, false);
            let range;
            if (find === null) {
                range = {
                    startLineNumber: position.lineNumber,
                    startColumn: position.column,
                    endLineNumber: position.lineNumber,
                    endColumn: position.column
                };
            } else {
                range = find.range;
            }
            let prev_range = {
                startLineNumber: range.startLineNumber,
                startColumn: 0,
                endLineNumber: range.startLineNumber,
                endColumn: range.startColumn - 1
            }
            let prev_word = model.findMatches(/[\w\.]+/, prev_range, true, true, null, false, 1);
            if (prev_word.length > 0) {
                items_without_operator.forEach(item => item.range = range);
                return { suggestions: items_without_operator }
            } else {
                all_items.forEach(item => item.range = range);
                return { suggestions: all_items }
            }
        }
    }
}

function getRiscvHoverProvider() {
    return {
        provideHover: (model, position, token) => {
            let c = model.getValueInRange({
                startLineNumber: position.lineNumber,
                startColumn: position.column,
                endLineNumber: position.lineNumber,
                endColumn: position.column + 1
            });
            if (!/[\w\.]*/.test(c)) {
                return null;
            }
            let prev_range = model.findPreviousMatch(/[\w\.]*/, position, true, true, null, false);
            let next_range = model.findNextMatch(/[\w\.]*/, position, true, true, null, false);
            let range = {
                startLineNumber: prev_range.range.startLineNumber,
                startColumn: prev_range.range.startColumn,
                endLineNumber: next_range.range.endLineNumber,
                endColumn: next_range.range.endColumn
            };
            let word = model.getValueInRange(range);
            let register = rv32i.register[word];
            let operator_list = rv32i.operator[word];
            let directive = rv32i.directive.includes(word) ? word : undefined;
            let title, detail;
            let markdown_string_list = [];
            if (register !== undefined) {
                title = `Register: ${word}`;
                detail = [register];
            } else if (operator_list !== undefined) {
                title = `Operator: ${word}`;
                detail = operator_list;
            } else if (directive !== undefined) {
                title = `Directive: ${word}`;
                detail = [word];
            }
            if (title !== undefined) {
                markdown_string_list.push({ value: `**${title}**` });
                markdown_string_list.push(...detail.map(d => ({ value: `\`${d}\`` })));
                console.log(markdown_string_list);
            }
            return {
                contents: markdown_string_list,
                range: range
            };
        }
    }
}