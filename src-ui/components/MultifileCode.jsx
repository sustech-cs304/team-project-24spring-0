import Code from '@/components/Code'
import { Tab, Tabs } from '@nextui-org/react'
import { Button, ButtonGroup } from '@nextui-org/react'
import { invoke } from '@tauri-apps/api/tauri'
import handleStimulatorResult from '@/utils/handleStimulatorResult'

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
    let result = await invoke('debug')
    console.log('Invoke handle debug result: ', result)

    await handleStimulatorResult(result, 'Debug', state, outputStore)
  }

  const handleRun = async () => {
    let result = await invoke('run')
    console.log('Invoke handle run result: ', result)

    await handleStimulatorResult(result, 'Run', state, outputStore)
  }

  var handleStep = async () => {
    console.log('Step Executed')
    const result = await invoke('step')
    console.log(result)

    await handleStimulatorResult(result, 'Step', state, outputStore)
  }

  var handleResume = async () => {
    console.log('Resume Executed')
    const result = await invoke('resume')
    console.log(result)

    await handleStimulatorResult(result, 'Resume', state, outputStore)
  }

  var handleReset = async () => {
    console.log('Reset Executed')
    const result = await invoke('reset')
    console.log(result)

    await handleStimulatorResult(result, 'Reset', state, outputStore)
  }

  var handleUndo = async () => {
    console.log('Undo Executed')
    const result = await invoke('undo')
    console.log(result)

    await handleStimulatorResult(result, 'Undo', state, outputStore)
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
                <Button color="primary" size="sm" className="w-full" onClick={() => handleStep()}>
                  Step
                </Button>
                <Button color="secondary" size="sm" className="w-full" onClick={() => handleResume()}>
                  Resume
                </Button>
                <Button color="primary" size="sm" className="w-full" onClick={() => handleUndo()}>
                  Undo
                </Button>
                <Button color="secondary" size="sm" className="w-full" onClick={() => handleReset()}>
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
