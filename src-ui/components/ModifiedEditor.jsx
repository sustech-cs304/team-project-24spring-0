import Editor, { useMonaco } from "@monaco-editor/react";
import React, { useEffect, useState } from "react";
import Image from "next/image";

import useFileStore from "@/utils/state";
import rv32i from "@/../constants/riscv/rv32i.json"


export default function ModifiedEditor({ fileName }) {
    const language_id = 'riscv';
    const monaco = useMonaco();
    const state = useFileStore();
    const file = useFileStore(state => state.files.find(file => file.fileName === fileName));
    useEffect(() => {
        if (monaco) {
            monaco.languages.register({ id: language_id });

            monaco.languages.setMonarchTokensProvider(language_id, {
                seperator: /[,:\s]/,

                register: rv32i.register,
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
                                '@default': 'symbol'
                            }
                        }],
                        [new RegExp(`(${rv32i.directive.join('|').replace(/\./g, '\\.')})(?=@seperator|$)`), 'directive'],
                        [/[^,:\s][\.\w]*(?=\W|$)/, 'unknown']
                    ],
                }
            });

            monaco.languages.registerCompletionItemProvider(language_id, {
                provideCompletionItems: (model, position, context, token) => {
                    return { suggestions: [] }
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
                    { token: 'symbol', foreground: '5BACE4' },
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
    }, []);

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