'use client';

import MultifileCode from "@/components/MultifileCode";
import Register from "@/components/Register";
import MessageIO from "@/components/MessageIO";
import {Card, CardBody, Textarea} from "@nextui-org/react";
import { useEffect } from "react";
import { listen } from '@tauri-apps/api/event';
import useFileStore from "@/utils/state";

export default function Home() {
    const state = useFileStore();
    const files = useFileStore((state) => state.files);

    useEffect(() => {
        const unListenedFileOpen = listen('front_file_open', (event) => {
            // setOutput(prevOutput => prevOutput + '\nEvent received:\n' + JSON.stringify(event.payload));

            state.addFile(
                {
                    fileName: event.payload["file_path"],
                    code: event.payload["content"],
                }
            );
            return event.payload;
        });


        return () => {
            unListenedFileOpen.then(dispose => dispose());
        };
    }, []);

  return (
      <main className='h-[calc(100vh-45px)]'>
          <div className='grid grid-cols-7 gap-4 p-2 max-h-[calc(100vh-45px)] w-full'>

              <div className='col-span-5 '>

                  <div className='grid grid-rows-8 gap-4 max-h-[calc(100vh-45px)] h-screen grow'>

                      <div className='row-span-5'>
                          <Card className='h-full w-full'>
                              <CardBody className='h-full w-full overflow-y-auto'>
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
