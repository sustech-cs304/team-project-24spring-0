'use client'

import React from 'react'

import {
  Button,
  ButtonGroup,
  Card,
  CardBody,
  CardHeader,
  Textarea,
  Tabs,
  Tab,
  Input,
  Divider,
  CardFooter,
} from '@nextui-org/react'
import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import useOutputStore from '@/utils/outputState'

export default function Share() {
  var [inSession, setInSession] = useState(false)
  var [port, setPort] = useState('')
  var [password, setPassword] = useState('')

  var sessionID = '123456'
  function handleSession() {
    setInSession(!inSession)
  }

  const handlePortChange = value => {
    var inputPort = parseInt(value)
    if ((inputPort <= 65535 && inputPort >= 0) || isNaN(inputPort)) {
      setPort(inputPort)
    } else {
      alert('Port should be within 0 and 65535')
    }
  }

  const handlePasswordChange = value => {
    setPassword(value)
  }

  const onServerButtonClick = async () => {
    if (inSession) {
      // end server
      var result = await invoke('stop_share_server')
      console.log(result)
      if (result) {
        setInSession(!inSession)
        alert('Stop share server successfully!')
      } else {
        alert('Failed to stop share sever.')
      }
    } else {
      // check whether port and pwd are neither null
      if (isNaN(port) || password == '') {
        return
      }
      // start server
      var result = await invoke('start_share_server', {
        port: port,
        password: password,
      })
      console.log(result)
      if (result.success) {
        setInSession(!inSession)
        alert('Start share server successfully!')
      } else {
        alert('Failed to start share sever. Reason: ' + result.message + '.')
      }
    }
  }

  return (
    <Card className="h-screen w-full">
      <CardHeader>
        <h1>Code Together with Your Friends!</h1>
      </CardHeader>
      <CardBody>
        <Tabs aria-label="Share">
          <Tab key="Share" title="Share">
            <Card>
              <CardBody>
                <div className="w-full flex flex-col gap-4">
                  <div className="flex w-full flex-wrap md:flex-nowrap mb-6 md:mb-0 gap-4">
                    <Input
                      type="number"
                      label="Port"
                      onValueChange={handlePortChange}
                      value={port}
                    />
                  </div>
                  <div className="flex w-full flex-wrap md:flex-nowrap mb-6 md:mb-0 gap-4">
                    <Input
                      type="password"
                      label="Password"
                      onValueChange={handlePasswordChange}
                      value={password}
                    />
                  </div>
                </div>
              </CardBody>
              <Divider />
              <CardFooter>
                <Button
                  className="py-2 w-full"
                  color="primary"
                  onClick={() => onServerButtonClick()}
                >
                  {inSession ? 'End Server' : 'Start Server'}
                </Button>
              </CardFooter>
            </Card>
          </Tab>
          <Tab key="Join" title="Join">
            <Textarea placeholder="Enter Session ID" className="py-4" />
            <Button className="p-2 w-full" color="primary" onClick={() => handleSession()}>
              {inSession ? 'Join Session' : 'Leave Session'}
            </Button>
          </Tab>
        </Tabs>
      </CardBody>
    </Card>
  )
}
