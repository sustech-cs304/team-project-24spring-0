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
} from '@nextui-org/react'
import { useState } from 'react'

export default function Share() {
  var [inSession, setInSession] = useState(false)

  var sessionID = '123456'
  function handleSession() {
    setInSession(!inSession)
  }

  return (
    <Card className="h-screen w-full">
      <CardHeader>
        <h1>Code Together with Your Friends!</h1>
      </CardHeader>
      <CardBody>
        <Tabs aria-label="Share">
          <Tab key="Share" title="Share">
            <Button className="p-2 w-full" color="primary" onClick={() => handleSession()}>
              {inSession ? 'End Session' : 'Start Session'}
            </Button>
            {inSession ? <Textarea className="py-4" value="Session ID" /> : <></>}
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
