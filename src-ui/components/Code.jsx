import React from "react";
import {Tabs, Tab, Card, CardBody, Textarea} from "@nextui-org/react";

export default function Code() {
    return (
        <div className="flex flex-col h-full">
            <Tabs aria-label="Options" className='h-full'>
                <Tab key="edit" title="Edit" className='h-full'>
                    <Card className='h-full'>
                        <CardBody className='h-full'>
                            <Textarea className='h-full'/>
                        </CardBody>
                    </Card>
                </Tab>
                <Tab key="excecute" title="Excecute">
                    <Card>
                        <CardBody>
                            Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
                        </CardBody>
                    </Card>
                </Tab>
            </Tabs>
        </div>
    );
}
