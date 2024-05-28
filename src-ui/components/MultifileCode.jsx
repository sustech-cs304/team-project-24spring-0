import Code from '@/components/Code'
import { Tab, Tabs } from '@nextui-org/react'
import { Button, ButtonGroup } from '@nextui-org/react'
import { invoke } from '@tauri-apps/api/tauri'

import useFileStore from '@/utils/state'
import useOutputStore from '@/utils/outputState'

export default function MultifileCode() {
  const state = useFileStore()
  const files = useFileStore(state => state.files)
  const outputStore = useOutputStore.getState()

  const deleteFile = async fileName => {
    state.deleteFile(fileName)
    const result = await invoke('close_tab', { filepath: fileName })
    if (result.success) {
      const outputStore = useOutputStore.getState()
      outputStore.addOutput('File closed: ' + fileName)
    }
  }

  const handleAssembly = async fileName => {
    let result = await invoke('assembly')
    // let result = {
    //     Success: {
    //         data: [],
    //         text: [
    //             'add x1, x2, x3',
    //             'add x2, x1, x4',
    //         ]
    //     },
    // };
    console.log('Invoke handle assembly result: ', result)
    if (result.Success) {
      const outputStore = useOutputStore.getState()
      outputStore.addOutput('Assembly Successfully!');
      const currentFile = state.files.find(file => file.fileName === fileName)
      state.updateFile(
        currentFile.fileName,
        currentFile.code,
        currentFile.original,
        result.Success.text,
        [],
        currentFile.register,
        currentFile.memory,
        currentFile.baseAddress,
      )
      console.log('updated file')
      console.log(currentFile)
    }
    if (result.Error) {
      const outputStore = useOutputStore.getState()
      var i = 0
      for (var error of result.Error) {
        outputStore.addOutput(
          'Error ' + i + ' at line ' + error.line + ', column ' + error.column + ': ' + error.msg,
        )
      }
    }
  }

  const handleSimulatorOperation = async (name) => {
    const result = await invoke(name);
    console.log(name, result);

    if (result.success) {
      outputStore.addOutput(name + ' Succeded!')
    } else {
      outputStore.addOutput(name + ' Failed! Reason: ' + result.message);
    }
  }

  const getRunButtonisDisabled = () => {
    const currentFile = state.files.find(file => file.fileName === state.currentFile);
    if (currentFile && currentFile.assembly.length != 0) {
        console.log(currentFile)
        console.log(currentFile.assembly.length)
        return false;
    }
    return true;
  }

  const getDebugButtonisDisabled = () => {
    const currentFile = state.files.find(file => file.fileName === state.currentFile);
    if (currentFile && currentFile.assembly.length != 0) {
      return false;
    }
    return true;
  }

  return (
    <Tabs size="small" aria-label="Files">
      {files.map(file => (
        <Tab
          key={file.fileName}
          title={
            file.fileName.split('/').pop().split('\\').pop() +
            (file.code != file.original ? ' *' : '')
          }
          className="h-full"
        >
          <div className="h-full w-full relative">
            <Code fileName={file.fileName} />
            <div className="absolute right-4 top-2 flex-row gap-2">
              <ButtonGroup>
                <Button color="success" size="sm" onClick={() => handleAssembly(file.fileName)}>
                  Assembly
                </Button>
                <Button color="primary" size="sm" isDisabled={getRunButtonisDisabled()} onClick={() => handleSimulatorOperation("run")}>
                  Run
                </Button>
                <Button color="secondary" size="sm" isDisabled={getDebugButtonisDisabled()} onClick={() => handleSimulatorOperation("debug")}>
                  Debug
                </Button>
                <Button color="primary" size="sm" className="w-full" onClick={() => handleSimulatorOperation("step")}>
                  Step
                </Button>
                <Button color="secondary" size="sm" className="w-full" onClick={() => handleSimulatorOperation("resume")}>
                  Resume
                </Button>
                <Button color="primary" size="sm" className="w-full" onClick={() => handleSimulatorOperation("undo")}>
                  Undo
                </Button>
                <Button color="secondary" size="sm" className="w-full" onClick={() => handleSimulatorOperation("reset")}>
                  Reset
                </Button>
                <Button color="danger" size="sm" onClick={() => deleteFile(file.fileName)}>
                  Close
                </Button>
              </ButtonGroup>
            </div>
          </div>
        </Tab>
        // <Tab key={file.fileName} title={file.fileName + (file.code!=file.original?' *':"")} className="h-full">
        //     <Code fileName={file.fileName}/>
        // </Tab>
      ))}
    </Tabs>
  )
}
