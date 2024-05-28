'use client'

import MultifileCode from '@/components/MultifileCode'
import Register from '@/components/Register'
import MessageIO from '@/components/MessageIO'
import { Card, CardBody, Textarea } from '@nextui-org/react'
import { useEffect } from 'react'
import { listen } from '@tauri-apps/api/event'
import useFileStore from '@/utils/state'
import useOutputStore from '@/utils/outputState'

export default function Home() {
  useEffect(() => {
    const unListenedFileOpen = listen('front_file_open', event => {
      console.log('file open event received')
      // setOutput(prevOutput => prevOutput + '\nEvent received:\n' + JSON.stringify(event.payload));
      const state = useFileStore.getState()
      for (let file of state.files) {
        if (file.fileName === event.payload['file_path']) {
          return
        }
      }
      console.log('file open event received')

      state.addFile({
        fileName: event.payload['file_path'],
        code: event.payload['content'],
        original: event.payload['content'],
        assembly: [],
        runLines: '',
        register: [
          { name: 'zero', number: '0', value: 0 },
          { name: 'ra', number: '1', value: 0 },
          { name: 'sp', number: '2', value: 0 },
          { name: 'gp', number: '3', value: 0 },
          { name: 'tp', number: '4', value: 0 },
          { name: 't0', number: '5', value: 0 },
          { name: 't1', number: '6', value: 0 },
          { name: 't2', number: '7', value: 0 },
          { name: 's0', number: '8', value: 0 },
          { name: 's1', number: '9', value: 0 },
          { name: 'a0', number: '10', value: 0 },
          { name: 'a1', number: '11', value: 0 },
          { name: 'a2', number: '12', value: 0 },
          { name: 'a3', number: '13', value: 0 },
          { name: 'a4', number: '14', value: 0 },
          { name: 'a5', number: '15', value: 0 },
          { name: 'a6', number: '16', value: 0 },
          { name: 'a7', number: '17', value: 0 },
          { name: 's2', number: '18', value: 0 },
          { name: 's3', number: '19', value: 0 },
          { name: 's4', number: '20', value: 0 },
          { name: 's5', number: '21', value: 0 },
          { name: 's6', number: '22', value: 0 },
          { name: 's7', number: '23', value: 0 },
          { name: 's8', number: '24', value: 0 },
          { name: 's9', number: '25', value: 0 },
          { name: 's10', number: '26', value: 0 },
          { name: 's11', number: '27', value: 0 },
          { name: 't3', number: '28', value: 0 },
          { name: 't4', number: '29', value: 0 },
          { name: 't5', number: '30', value: 0 },
          { name: 't6', number: '31', value: 0 },
          { name: 'pc', number: '32', value: 0 },
        ],
        memory: [
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0,
        ],
        baseAddress: 0x10010000,
      })
      // return event.payload;
    })

    const unListenedFileSave = listen('front_file_save', event => {
      const state = useFileStore.getState()
      const file = state.files.find(file => file.fileName === state.currentFile)
      state.updateFile(
        state.currentFile,
        file.code,
        file.code,
        file.assembly,
        file.runLines,
        file.register,
        file.memory,
        file.baseAddress,
      )
    })

    const unListenedFileSaveAs = listen('front_file_save_as', event => {
      const state = useFileStore.getState()
      const file = state.files.find(file => file.fileName === state.currentFile)
      state.updateFile(
        state.currentFile,
        file.code,
        file.code,
        file.assembly,
        file.runLines,
        file.register,
        file.memory,
        file.baseAddress,
      )
    })

    const unListenedSimulatorUpdate = listen('front_simulator_update', event => {
      // the payload is a SimulatorData containing the current pc index, register and memory values.
      //
      //     SimulatorData:
      //
      // filepath: string
      // success: bool
      // has_current_text: bool
      // current_text: u64
      // registers: Vec<Register>
      // data: Vec
      // message: string
      console.log('simulator update event received', event.payload)
      const outputStore = useOutputStore.getState()
      if (event.payload['success'] === false) {
        outputStore.addOutput('Simulator Update Failed. Message: ' + event.payload['message'])
      } else {
        outputStore.addOutput('Simulator Updated Successfully')
      }

      const state = useFileStore.getState()
      const file = state.files.find(file => file.fileName === state.currentFile)
      file.register = event.payload['registers']
      file.memory = event.payload['data']
      if (event.payload['has_current_text']) {
        file.runLines = event.payload['current_text']
      } else {
        file.runLines = ''
      }
      state.updateFile(
        state.currentFile,
        file.code,
        file.code,
        file.assembly,
        file.runLines,
        file.register,
        file.memory,
        file.baseAddress,
      )
    })

    return () => {
      unListenedFileOpen.then(dispose => dispose())
      unListenedFileSave.then(dispose => dispose())
      unListenedFileSaveAs.then(dispose => dispose())
      unListenedSimulatorUpdate.then(dispose => dispose())
    }
  }, [])

  return (
    <main className="h-[calc(100vh-45px)]">
      <div className="grid grid-cols-7 gap-4 p-2 max-h-[calc(100vh-45px)] w-full">
        <div className="col-span-5 ">
          <div className="grid grid-rows-8 gap-4 max-h-[calc(100vh-45px)] h-screen grow">
            <div className="row-span-5">
              <Card className="h-full w-full">
                <CardBody className="h-full w-full overflow-y-auto overflow-x-auto">
                  <MultifileCode />
                </CardBody>
              </Card>
            </div>

            <div className="row-span-3">
              <Card className="h-full w-full">
                <CardBody className="h-full w-full">
                  <MessageIO />
                </CardBody>
              </Card>
            </div>
          </div>
        </div>

        <div className="col-span-2">
          <Card className="h-full w-full">
            <CardBody className="h-full w-full">
              <Register />
            </CardBody>
          </Card>
        </div>
      </div>
    </main>
  )
}
