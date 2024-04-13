import Editor, {useMonaco} from "@monaco-editor/react";
import React, {useEffect} from "react";
import Image from "next/image";


export default function ModifiedEditor() {
    const monaco = useMonaco()
    useEffect(() => {
        if (monaco) {
            monaco.editor.defineTheme('myTheme', {
                base: 'vs-dark',
                inherit: true,
                rules: [
                    {
                        token: 'comment',
                        foreground: 'ffa500',
                        fontStyle: 'italic underline'
                    },
                    {
                        token: 'comment.js',
                        foreground: '008800',
                        fontStyle: 'bold'
                    },
                    {
                        token: 'comment.css',
                        foreground: '0000ff'
                    }
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
    }, [monaco]);

    return (
        <div className='h-full relative'>
            <Editor language='javascript' className='overflow-hidden h-full'/>
            <div className='absolute right-2 top-0 flex-row gap-2'>
                <button className='bg-gray-100 rounded-2xl hover:bg-gray-200'>
                    <Image src='/icons/run.svg' width={16} height={16}/>
                </button>
            </div>
        </div>

    );
}