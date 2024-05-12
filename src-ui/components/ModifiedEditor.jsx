import Editor, {useMonaco} from "@monaco-editor/react";
import React, {useEffect, useRef} from "react";
import Image from "next/image";
import { invoke } from '@tauri-apps/api/tauri';
import useOutputStore from "@/utils/outputState";
import useFileStore from "@/utils/state";

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
        const result = await invoke('update_tab', {filepath: fileName, data: value});
        // console.log('update tab result: ', result);
        if(!result.success){
            console.log('Error updating tab');
            const outputStore = useOutputStore.getState();
            outputStore.addOutput('Error updating tab: ' + fileName);
        }
        let position = editorRef.current.getPosition();
        let line = position.lineNumber;
        let column = position.column;
        console.log('Current: line: ', line, 'column: ', column, 'value: ', newInput);
    }

    return (
        <div className='h-full relative'>
            <Editor 
            language='javascript' 
            className='overflow-hidden h-full'
            value={file.code}
            onChange={handleEditorChange}
            onMount={handleEditorDidMount}
            />
            <div className='absolute right-2 top-0 flex-row gap-2'>
                <button className='bg-gray-100 rounded-2xl hover:bg-gray-200'>
                    <Image alt="run icon" src='/icons/run.svg' width={16} height={16}/>
                </button>
            </div>
        </div>

    );
}