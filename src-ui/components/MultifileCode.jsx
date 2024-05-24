import Code from '@/components/Code'
import { Tab, Tabs } from '@nextui-org/react'
import { Button, ButtonGroup } from '@nextui-org/react'
import { invoke } from '@tauri-apps/api/tauri'

import useFileStore from '@/utils/state'
import useOutputStore from '@/utils/outputState'

export default function MultifileCode() {
  const state = useFileStore()
  const files = useFileStore(state => state.files)

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
      outputStore.addOutput('Assembly Result: \n' + result.Success.text.join('\n'))
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

  const handleDebug = async () => {
    let result = await invoke('debug');
    const outputStore = useOutputStore.getState()
    console.log('Invoke handle debug result: ', result)

    if (result.success) {
      outputStore.addOutput('Debug Succeeded!')
      let fileName = state.currentFile;
      const currentFile = state.files.find(file => file.fileName === fileName)
      state.updateFile(
          currentFile.fileName,
          currentFile.code,
          currentFile.original,
          currentFile.assembly,
          currentFile.runLines,
          result.registers,
          currentFile.memory,
          currentFile.baseAddress,
      )
      if (result.has_current_text) {
        state.updateFile(
            currentFile.fileName,
            currentFile.code,
            currentFile.original,
            currentFile.assembly,
            [result.current_text],
            result.registers,
            currentFile.memory,
            currentFile.baseAddress,
        )
      }
      console.log('updated file')
      console.log(currentFile)
    } else {
      outputStore.addOutput('Debug Failed!')
    }

    if (result.has_message) {
      outputStore.addOutput('Debug Result: \n' + result.message)
    }
  }

  const handleRun = async () => {
    let result = await invoke('run');
    const outputStore = useOutputStore.getState();
    console.log('Invoke handle run result: ', result);

    if (result.success) {
      outputStore.addOutput('Run Succeded!')
      let fileName = state.currentFile
      const currentFile = state.files.find(file => file.fileName === fileName)
      state.updateFile(
        currentFile.fileName,
        currentFile.code,
        currentFile.original,
        currentFile.assembly,
        currentFile.runLines,
        result.registers,
        currentFile.memory,
        currentFile.baseAddress,
      )
      if (result.has_current_text) {
        state.updateFile(
          currentFile.fileName,
          currentFile.code,
          currentFile.original,
          currentFile.assembly,
          [result.current_text],
          result.registers,
          currentFile.memory,
          currentFile.baseAddress,
        )
      }
      console.log('updated file')
      console.log(currentFile)
    } else {
      outputStore.addOutput('Run Failed!')
    }
    if (result.has_message) {
      outputStore.addOutput('Run Result: \n' + result.message)
    }
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
                <Button color="primary" size="sm" onClick={() => handleRun()}>
                  Run
                </Button>
                <Button color="secondary" size="sm" onClick={() => handleDebug()}>
                  Debug
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
