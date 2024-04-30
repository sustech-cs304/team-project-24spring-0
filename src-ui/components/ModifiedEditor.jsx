import Editor, { useMonaco } from "@monaco-editor/react";
import React, { useEffect } from "react";
import Image from "next/image";

import useFileStore from "@/utils/state";
import rv32i from "@/constants/riscv/rv32i.json"

let config_loaded = false;
const language_id = 'riscv';

export default function ModifiedEditor({ fileName }) {
    const monaco = useMonaco();
    const state = useFileStore();
    const file = useFileStore(state => state.files.find(file => file.fileName === fileName));
    useEffect(() => {
        if (monaco) {
            if (!config_loaded) {
                LoadMonacoConfig(monaco);
            }
        }
    }, [monaco]);

    return (
        <div className='h-full relative'>
            <Editor
                language={language_id}
                theme={language_id}
                className='overflow-hidden h-full'
                value={file.code}
                onChange={(value) => state.updateFile(fileName, value)}
            />
            <div className='absolute right-2 top-0 flex-row gap-2'>
                <button className='bg-gray-100 rounded-2xl hover:bg-gray-200'>
                    <Image src='/icons/run.svg' width={16} height={16} />
                </button>
            </div>
        </div>

    );
}

function LoadMonacoConfig(monaco) {
    config_loaded = true;
    monaco.languages.register({ id: language_id });

    let directive = rv32i.directive;
    let register_map = rv32i.register;
    let register_key = Object.keys(register_map);
    let operator_map = rv32i.operator;
    let operator_key = Object.keys(operator_map);

    monaco.languages.setMonarchTokensProvider(language_id, {
        seperator: /[,:\s]/,

        register: register_key,
        operator: operator_key,

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
    });

    let directive_items = directive.map(directive => {
        return {
            label: directive,
            kind: monaco.languages.CompletionItemKind.Keyword,
            detail: directive,
            range: null,
            insertText: directive
        }
    });

    let register_items = register_key.map(register => {
        return {
            label: register,
            kind: monaco.languages.CompletionItemKind.Value,
            detail: `Register ${register_map[register]}`,
            range: null,
            insertText: register
        }
    });

    let operator_items = [];
    for (let operator of operator_key) {
        if (operator_map[operator].length === 0) {
            operator_items.push({
                label: operator,
                kind: monaco.languages.CompletionItemKind.Operator,
                detail: 'unimplemented',
                range: null,
                insertText: operator
            });
        } else {
            for (let hint of operator_map[operator]) {
                operator_items.push({
                    label: operator,
                    kind: monaco.languages.CompletionItemKind.Operator,
                    detail: hint,
                    range: null,
                    insertText: operator
                });
            }
        }
    }

    let all_items = directive_items.concat(register_items).concat(operator_items);

    monaco.languages.registerCompletionItemProvider(language_id, {
        provideCompletionItems: (model, position, context, token) => {
            let find = model.findPreviousMatch(/[\w\.]*/, position, true, true, false, false);
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
            all_items.forEach(item => item.range = range);
            return { suggestions: all_items }
        }
    });

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
