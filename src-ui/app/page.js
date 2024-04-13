'use client';

import MultifileCode from "@/components/MultifileCode";
import Register from "@/components/Register";
import MessageIO from "@/components/MessageIO";
import {Card, CardBody, Textarea} from "@nextui-org/react";

export default function Home() {
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
