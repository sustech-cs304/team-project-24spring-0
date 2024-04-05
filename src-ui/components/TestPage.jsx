import {Input, Button, Textarea} from "@nextui-org/react";
import {invoke} from '@tauri-apps/api/tauri';
import {useState} from 'react';
import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';

export default function TestPage() {
    const [inputValue, setInputValue] = useState('');
    const [output, setOutput] = useState('');

    const handleInputChange = (event) => {
        setInputValue(event.target.value);
    };

    const handleClick = async () => {
        try {
            const result = await invoke('read_file', {val : inputValue});
            setOutput('===result===\n' + result + '\n===    type===\n' + typeof (result));
        } catch (error) {
            console.error('Error calling Tauri command:', error);
            setOutput('Error occurred: ' + error);
        }
    };

    return (
        <div className='items-center gap-4'>
            <div className='flex flex-col p-2'>
                <Input
                    id="input"
                    type="text"
                    label="Input"
                    className='p-2'
                    value={inputValue}
                    onChange={handleInputChange}/>
                <Button color='primary' onClick={handleClick}>Run</Button>
            </div>
            <textarea value={output} className='h-full bg-gray-50 pt-2 w-full rounded-2xl' readOnly/>
        </div>
    );
}
