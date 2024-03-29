'use client';

import Taskbar from "@/components/Taskbar";
import Code from "@/components/Code";
import Register from "@/components/Register";
import {Card, CardBody, Textarea} from "@nextui-org/react";

export default function Home() {
  return (
      <main className='h-[calc(100vh-50px)] grow'>
          <Taskbar/>
          <div className='grid grid-cols-7 gap-2 p-2 h-full w-full'>

              <div className='col-span-5'>

                  <div className='grid grid-rows-8 gap-2 h-full grow'>

                      <div className='row-span-5'>
                          <Card className='h-full w-full'>
                              <CardBody className='h-full w-full'>
                                  <Code/>
                              </CardBody>
                          </Card>
                      </div>

                      <div className='row-span-3'>
                          <Card className='h-full w-full'>
                              <CardBody className='h-full w-full'>
                                  <Code/>
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
