import Code from "@/components/Code";
import {Button, ButtonGroup, Tab, Tabs} from "@nextui-org/react";
import {invoke} from '@tauri-apps/api/tauri';

import useFileStore from "@/utils/state";
import useOutputStore from "@/utils/outputState";

export default function MultifileCode() {
    const state = useFileStore();
    const files = useFileStore(state => state.files);

    const deleteFile = async (fileName) => {
        state.deleteFile(fileName);
        const result = await invoke('close_tab', {filepath: fileName});
        if (result.success) {
            const outputStore = useOutputStore.getState();
            outputStore.addOutput('File closed: ' + fileName);
        }
    }

    const handleAssembly = async (fileName) => {
        const result = await invoke('read_tab', {filepath: fileName});
        const file = state.files.find(file => file.fileName === fileName);
        const outputStore = useOutputStore.getState();
        outputStore.addOutput('Assembly Result: \n' + result.message);
        // if message does not start with error
        if (!result.message.startsWith('Error')) {
            state.updateFile(fileName, file.code, file.original, result.message, file.runLines)
        }
    }

    const handleDebug = async () => {
        const result = await invoke('debug');
        const outputStore = useOutputStore.getState();
        console.log('Invoke handle debug result: ', result);
        outputStore.addOutput('Debug Result: \n' + result.message);
    }

    return (
        <Tabs size="small" aria-label="Files">
            {files.map(file => (
                <Tab key={file.fileName}
                     title={file.fileName.split('/').pop().split('\\').pop() + (file.code !== file.original ? ' *' : "")}
                     className="h-full">
                    <div className="h-full w-full relative">
                        <Code fileName={file.fileName}/>
                        <div className='absolute right-4 top-2 flex-row gap-2'>
                            <ButtonGroup>
                                <Button color="success" size="sm"
                                        onClick={() => handleAssembly(file.fileName)}>Assembly</Button>
                                <Button color="success" size="sm" onClick={() => handleDebug()}>Debug</Button>
                                <Button color="danger" size="sm"
                                        onClick={() => deleteFile(file.fileName)}>Close</Button>
                            </ButtonGroup>
                        </div>
                    </div>
                </Tab>
                // <Tab key={file.fileName} title={file.fileName + (file.code!=file.original?' *':"")} className="h-full">
                //     <Code fileName={file.fileName}/>
                // </Tab>
            ))}
        </Tabs>
    );
}