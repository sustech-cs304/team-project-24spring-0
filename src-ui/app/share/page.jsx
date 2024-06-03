'use client';

import React from 'react';

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
} from '@nextui-org/react';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import useOutputStore from '@/utils/outputState';

export default function Share() {
  var [inSession, setInSession] = useState(false);
  var [port, setPort] = useState('');
  var [password, setPassword] = useState('');
  var [ip, setIp] = useState('');
  var [port2, setPort2] = useState('');
  var [password2, setPassword2] = useState('');

  var sessionID = '123456';
  function handleSession() {
    setInSession(!inSession);
  }

  const handlePortChange = value => {
    var inputPort = parseInt(value);
    if ((inputPort <= 65535 && inputPort >= 0) || isNaN(inputPort)) {
      setPort(inputPort);
    } else {
      alert('Port should be within 0 and 65535');
    }
  };

  const handlePasswordChange = value => {
    setPassword(value);
  };

  const handleIpChange = value => {
    setIp(value);
  };

  const handlePort2Change = value => {
    var inputPort = parseInt(value);
    if ((inputPort <= 65535 && inputPort >= 0) || isNaN(inputPort)) {
      setPort2(inputPort);
    } else {
      alert('Port should be within 0 and 65535');
    }
  };

  const handlePassword2Change = value => {
    setPassword2(value);
  };

  const onServerButtonClick = async () => {
    if (inSession) {
      // end server
      var result = await invoke('stop_share_server');
      console.log(result);
      if (result) {
        setInSession(!inSession);
        alert('Stop share server successfully!');
      } else {
        alert('Failed to stop share sever.');
      }
    } else {
      // check whether port and pwd are neither null
      if (isNaN(port) || password == '') {
        alert('Port and password should not be empty!');
        return;
      }
      // start server
      var result = await invoke('start_share_server', {
        port: port,
        password: password,
      });
      console.log(result);
      if (result.success) {
        setInSession(!inSession);
        alert('Start share server successfully!');
      } else {
        alert('Failed to start share sever. Reason: ' + result.message + '.');
      }
    }
  };

  const onAuthorize = async () => {
    if (ip == '' || isNaN(port2) || password2 == '') {
      alert('IP, port and password should not be empty!');
      return;
    }

    // check whether ip is a valid ipv4 address
    var ipPattern = new RegExp(
      '^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$',
    );
    if (!ipPattern.test(ip)) {
      alert('IP is not a valid ipv4 address!');
      return;
    }

    var result = await invoke('authorize_share_client', {
      ip: ip,
      port: port2,
      password: password2,
    });
    console.log(result);
    if (result.success) {
      setInSession(!inSession);
      alert('Authorize successfully!');
    } else {
      alert('Failed to authorize. Reason: ' + result.message + '.');
    }
  };

  return (
    <Card className='h-screen w-full'>
      <CardHeader>
        <h1>Code Together with Your Friends!</h1>
      </CardHeader>
      <CardBody>
        <Tabs aria-label='Share'>
          <Tab key='Share' title='Share'>
            <Card>
              <CardBody>
                <div className='w-full flex flex-col gap-4'>
                  <div className='flex w-full flex-wrap md:flex-nowrap mb-6 md:mb-0 gap-4'>
                    <Input
                      type='number'
                      label='Port'
                      onValueChange={handlePortChange}
                      value={port}
                    />
                  </div>
                  <div className='flex w-full flex-wrap md:flex-nowrap mb-6 md:mb-0 gap-4'>
                    <Input
                      type='password'
                      label='Password'
                      onValueChange={handlePasswordChange}
                      value={password}
                    />
                  </div>
                </div>
              </CardBody>
              <Divider />
              <CardFooter>
                <Button
                  className='py-2 w-full'
                  color='primary'
                  onClick={() => onServerButtonClick()}
                >
                  {inSession ? 'End Server' : 'Start Server'}
                </Button>
              </CardFooter>
            </Card>
          </Tab>
          <Tab key='Join' title='Join'>
            <Card>
              <CardBody>
                <div className='w-full flex flex-col gap-4'>
                  <div className='flex w-full flex-wrap md:flex-nowrap mb-6 md:mb-0 gap-4'>
                    <Input
                      type='text'
                      label='IP'
                      onValueChange={handleIpChange}
                      value={ip}
                    />
                  </div>
                  <div className='flex w-full flex-wrap md:flex-nowrap mb-6 md:mb-0 gap-4'>
                    <Input
                      type='number'
                      label='Port'
                      onValueChange={handlePort2Change}
                      value={port2}
                    />
                  </div>
                  <div className='flex w-full flex-wrap md:flex-nowrap mb-6 md:mb-0 gap-4'>
                    <Input
                      type='password'
                      label='Password'
                      onValueChange={handlePassword2Change}
                      value={password2}
                    />
                  </div>
                </div>
              </CardBody>
              <Divider />
              <CardFooter>
                <Button
                  className='p-2 w-full'
                  color='primary'
                  onClick={() => onAuthorize()}
                >
                  Authorize
                </Button>
              </CardFooter>
            </Card>
          </Tab>
        </Tabs>
      </CardBody>
    </Card>
  );
}
