import React, { useEffect, useRef } from 'react'
import ModifiedEditor from '@/components/ModifiedEditor'
import { Tabs, Tab, Card, CardBody, Textarea } from '@nextui-org/react'
import TestPage from '@/components/TestPage'
import useFileStore from '@/utils/state'
import CodeLineTable from './CodeLineTable'
import { invoke } from '@tauri-apps/api/tauri'
import Memory from '@/components/Memory'

export default function Code({ fileName }) {
  const store = useFileStore()
  const initialized = useRef(false)

  useEffect(() => {
    if (!initialized.current) {
      initialized.current = true
      store.changeCurrentFile(fileName)
      invoke('change_current_tab', { newpath: fileName })
      console.log('changed current file to ' + fileName)
    }
  }, [])

  return (
    <div className="flex flex-col h-full">
      <Tabs aria-label="Options">
        <Tab key="edit" title="Edit" className="h-full">
          <Card className="h-full">
            <CardBody className="h-full">
              {/* <Tabs key="small" size="small" aria-label="Tabs sizes">
                                <Tab key="file1" title="file1.m" className="h-full"> <Editor language='javascript' className='overflow-hidden h-full'/> </Tab>
                                <Tab key="file2" title="file2.m" className="h-full"> <Editor language='javascript' className='overflow-hidden h-full'/> </Tab>
                                <Tab key="file3" title="file3.m" className="h-full"> <Editor language='javascript' className='overflow-hidden h-full'/> </Tab>
                            </Tabs> */}
              <ModifiedEditor fileName={fileName} />
            </CardBody>
          </Card>
        </Tab>
        <Tab key="excecute" title="Execute" className="h-full w-full">
          <Card className="h-full">
            <CardBody className="h-full flow grid-flow-row gap-4">
              <Tabs aria="execute" isVertical="true" size="sm">
                <Tab key="codeTable" title="Code Table">
                  <CodeLineTable fileName={fileName} />
                </Tab>
                <Tab key="memory" title="Memory">
                  <Memory fileName={fileName} />
                </Tab>
              </Tabs>
            </CardBody>
          </Card>
        </Tab>
      </Tabs>
    </div>
  )
}
