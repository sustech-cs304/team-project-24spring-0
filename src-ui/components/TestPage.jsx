import { Input, Button, Textarea } from '@nextui-org/react';
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import useFileStore from '@/utils/state';

export default function TestPage() {
  const [inputValue, setInputValue] = useState('');
  const [output, setOutput] = useState('');
  const state = useFileStore();

  const handleInputChange = event => {
    setInputValue(event.target.value);
  };

  const handleClick = async () => {
    try {
      const result = await invoke('read_tab', { filepath: inputValue });
      setOutput('===result===\n' + result + '\n===type===\n' + typeof result);
    } catch (error) {
      setOutput('Error occurred:\n' + error);
    }
  };

  useEffect(() => {
    const unListened = listen('front_file_open', event => {
      setOutput(
        prevOutput =>
          prevOutput + '\nEvent received:\n' + JSON.stringify(event.payload),
      );
    });

    return () => {
      unListened.then(dispose => dispose());
    };
  }, []);

  // useEffect(() => {
  //     const unListened = listen('front_file_open', (event) => {
  //         setOutput(prevOutput => prevOutput + '\nEvent received:\n' + JSON.stringify(event.payload));
  //         state.addFile(
  //             {
  //                 fileName: event.payload["file_path"],
  //                 code: event.payload["content"],
  //             }
  //         );
  //         return event.payload;
  //     });

  //     return () => {
  //         const result = unListened.then(dispose => dispose());
  //     };
  // }, []);

  return (
    <div className='items-center gap-4'>
      <div className='flex flex-col p-2'>
        <Input
          id='input'
          type='text'
          label='Input'
          className='p-2'
          value={inputValue}
          onChange={handleInputChange}
        />
        <Button color='primary' onClick={handleClick}>
          Run
        </Button>
      </div>
      <Textarea
        value={output}
        className='h-full bg-gray-50 pt-2 w-full rounded-2xl'
        readOnly
      />
    </div>
  );
}
