import Code from '@/components/CodeCard'
import { Tab, Tabs } from '@nextui-org/react'
import { Button, ButtonGroup } from '@nextui-org/react'
import { invoke } from '@tauri-apps/api/tauri'
import React, { useState } from 'react'

import useFileStore from '@/utils/state'
import useOutputStore from '@/utils/outputState'

export default function MultifileCode() {
  const state = useFileStore()
  const files = useFileStore(state => state.files)
  const outputStore = useOutputStore.getState()
  const [runStarted, setRunStarted] = useState(false)

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

    console.log('Invoke handle assembly result: ', result)
    if (result.Success) {
      const outputStore = useOutputStore.getState()
      outputStore.addOutput('Assembly Successfully!')
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

  const handleSimulatorOperation = async name => {
    const result = await invoke(name)
    console.log(name, result)

    if (result.success) {
      outputStore.addOutput(name + ' Succeded!')
      if (name === 'reset') {
        setRunStarted(false);
      } else if (name === `run` || name === `debug` || name === `step`) {
        setRunStarted(true);
      }
    } else {
      setRunStarted(false);
      outputStore.addOutput(name + ' Failed! Reason: ' + result.message)
    }
  }

  const getAssemblyButtonisDisabled = () => {
    const currentFile = state.files.find(file => file.fileName === state.currentFile)
    if (currentFile && currentFile.code.length != 0) {
      return false
    }
    return true
  }

  const getRunDebugStepButtonDisabled = () => {
    const currentFile = state.files.find(file => file.fileName === state.currentFile)
    if (currentFile && currentFile.assembly.length != 0) {
      if (runStarted && currentFile.runLines.length == 0) {
        return true;
      }
      return false;
    }
    return true;
  }

  const getResumeButtonisDisabled = () => {
    return false
  }

  const getUndoButtonisDisabled = () => {
    return !runStarted;
  }

  const getResetButtonisDisabled = () => {
    const currentFile = state.files.find(file => file.fileName === state.currentFile)
    if (currentFile && currentFile.assembly.length != 0) {
      return false
    }
    return true
  }

  const getCloseButtonisDisabled = () => {
    return false
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
                <Button color="success" size="sm" isDisabled={getAssemblyButtonisDisabled()} onClick={() => handleAssembly(file.fileName)}>
                  Assembly
                </Button>
                <Button
                  color="primary"
                  size="sm"
                  isDisabled={getRunDebugStepButtonDisabled()}
                  onClick={() => handleSimulatorOperation('run')}
                >
                  Run
                </Button>
                <Button
                  color="secondary"
                  size="sm"
                  isDisabled={getRunDebugStepButtonDisabled()}
                  onClick={() => handleSimulatorOperation('debug')}
                >
                  Debug
                </Button>
                <Button
                  color="primary"
                  size="sm"
                  className="w-full"
                  isDisabled={getRunDebugStepButtonDisabled()}
                  onClick={() => handleSimulatorOperation('step')}
                >
                  Step
                </Button>
                <Button
                  color="secondary"
                  size="sm"
                  className="w-full"
                  isDisabled={getResumeButtonisDisabled()}
                  onClick={() => handleSimulatorOperation('resume')}
                >
                  Resume
                </Button>
                <Button
                  color="primary"
                  size="sm"
                  className="w-full"
                  isDisabled={getUndoButtonisDisabled()}
                  onClick={() => handleSimulatorOperation('undo')}
                >
                  Undo
                </Button>
                <Button
                  color="secondary"
                  size="sm"
                  className="w-full"
                  isDisabled={getResetButtonisDisabled()}
                  onClick={() => handleSimulatorOperation('reset')}
                >
                  Reset
                </Button>
                <Button color="danger" size="sm" isDisabled={getCloseButtonisDisabled()} onClick={() => deleteFile(file.fileName)}>
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
