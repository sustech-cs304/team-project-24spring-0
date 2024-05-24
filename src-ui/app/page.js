'use client';

import MultifileCode from "@/components/MultifileCode";
import Register from "@/components/Register";
import MessageIO from "@/components/MessageIO";
import {Card, CardBody, Textarea} from "@nextui-org/react";
import { useEffect } from "react";
import { listen } from '@tauri-apps/api/event';
import useFileStore from "@/utils/state";

export default function Home() {

    useEffect(() => {
        const unListenedFileOpen = listen('front_file_open', (event) => {
            console.log('file open event received');
            // setOutput(prevOutput => prevOutput + '\nEvent received:\n' + JSON.stringify(event.payload));
            const state = useFileStore.getState();
            for (let file of state.files) {
                if (file.fileName === event.payload["file_path"]) {
                    return;
                }
            }
            console.log('file open event received');

            state.addFile(
                {
                    fileName: event.payload["file_path"],
                    code: event.payload["content"],
                    original: event.payload["content"],
                    assembly: [],
                    runLines: [],
                    register: [
                        {name: "x", number:'0', value: 0},
                        {name: "x", number:'1', value: 0},
                        {name: "x", number:'2', value: 0},
                        {name: "x", number:'3', value: 0},
                        {name: "x", number:'4', value: 0},
                        {name: "x", number:'5', value: 0},
                        {name: "x", number:'6', value: 0},
                        {name: "x", number:'7', value: 0},
                        {name: "x", number:'8', value: 0},
                        {name: "x", number:'9', value: 0},
                        {name: "x", number:'10', value: 0},
                        {name: "x", number:'11', value: 0},
                        {name: "x", number:'12', value: 0},
                        {name: "x", number:'13', value: 0},
                        {name: "x", number:'14', value: 0},
                        {name: "x", number:'15', value: 0},
                        {name: "x", number:'16', value: 0},
                        {name: "x", number:'17', value: 0},
                        {name: "x", number:'18', value: 0},
                        {name: "x", number:'19', value: 0},
                        {name: "x", number:'20', value: 0},
                        {name: "x", number:'21', value: 0},
                        {name: "x", number:'22', value: 0},
                        {name: "x", number:'23', value: 0},
                        {name: "x", number:'24', value: 0},
                        {name: "x", number:'25', value: 0},
                        {name: "x", number:'26', value: 0},
                        {name: "x", number:'27', value: 0},
                        {name: "x", number:'28', value: 0},
                        {name: "x", number:'29', value: 0},
                        {name: "x", number:'30', value: 0},
                        {name: "x", number:'31', value: 0},

                    ],
                    memory: [
                        [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
                        [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
                        [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
                        [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
                        [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
                        [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
                        [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
                        [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
                    ],
                    baseAddress: 0x10010000
                }
            );
            // return event.payload;
        });

        const unListenedFileSave = listen('front_file_save', (event) => {
            const state = useFileStore.getState();
            const file = state.files.find(file => file.fileName === state.currentFile);
            state.updateFile(state.currentFile, file.code, file.code, file.assembly, file.runLines, file.register, file.memory, file.baseAddress);
        });

        const unListenedFileSaveAs = listen('front_file_save_as', (event) => {
            const state = useFileStore.getState();
            const file = state.files.find(file => file.fileName === state.currentFile);
            state.updateFile(state.currentFile, file.code, file.code, file.assembly, file.runLines, file.register, file.memory, file.baseAddress);
        });


        return () => {
            unListenedFileOpen.then(dispose => dispose());
            unListenedFileSave.then(dispose => dispose());
            unListenedFileSaveAs.then(dispose => dispose());
        };
    }, []);

  return (
      <main className='h-[calc(100vh-45px)]'>
          <div className='grid grid-cols-7 gap-4 p-2 max-h-[calc(100vh-45px)] w-full'>

              <div className='col-span-5 '>

                  <div className='grid grid-rows-8 gap-4 max-h-[calc(100vh-45px)] h-screen grow'>

                      <div className='row-span-5'>
                          <Card className='h-full w-full'>
                              <CardBody className='h-full w-full overflow-y-auto overflow-x-auto'>
                                  <MultifileCode />
                              </CardBody>
                          </Card>
                      </div>

                      <div className='row-span-3'>
                          <Card className='h-full w-full'>
                              <CardBody className='h-full w-full'>
                                  <MessageIO/>
                              </CardBody>
                          </Card>
                      </div>

                  </div>

              </div>

              <div className='col-span-2'>
                  <Card className='h-full w-full'>
                      <CardBody className='h-full w-full'>
                          <Register/>
                      </CardBody>
                  </Card>
              </div>

          </div>
      </main>
  );
}
