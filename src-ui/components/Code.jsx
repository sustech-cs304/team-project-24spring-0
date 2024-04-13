import React, {useEffect} from "react";
import ModifiedEditor from "@/components/ModifiedEditor";
import {Tabs, Tab, Card, CardBody, Textarea} from "@nextui-org/react";
import {Table, TableHeader, TableBody, TableRow, TableColumn, TableCell} from "@nextui-org/react";
import TestPage from "@/components/TestPage";


export default function Code() {

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
                            <ModifiedEditor />
                        </CardBody>
                    </Card>
                </Tab>
                <Tab key="excecute" title="Excecute" className="h-full">
                    <Card className='h-full'>
                        <CardBody className="h-full flow grid-flow-row gap-4">
                            <Table aria-label="Example static collection table" className='row-span-1'>
                                <TableHeader>
                                    <TableColumn>NAME</TableColumn>
                                    <TableColumn>ROLE</TableColumn>
                                    <TableColumn>STATUS</TableColumn>
                                </TableHeader>
                                <TableBody>
                                    <TableRow key="1">
                                        <TableCell>Tony Reichert</TableCell>
                                        <TableCell>CEO</TableCell>
                                        <TableCell>Active</TableCell>
                                    </TableRow>
                                    <TableRow key="2">
                                        <TableCell>Zoey Lang</TableCell>
                                        <TableCell>Technical Lead</TableCell>
                                        <TableCell>Paused</TableCell>
                                    </TableRow>
                                    <TableRow key="4">
                                        <TableCell>William Howard</TableCell>
                                        <TableCell>Community Manager</TableCell>
                                        <TableCell>Vacation</TableCell>
                                    </TableRow>
                                    <TableRow key="5">
                                        <TableCell>William Howard</TableCell>
                                        <TableCell>Community Manager</TableCell>
                                        <TableCell>Vacation</TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>
                            <Table aria-label="Example static collection table" className='row-span-1'>
                                <TableHeader>
                                    <TableColumn>NAME</TableColumn>
                                    <TableColumn>ROLE</TableColumn>
                                    <TableColumn>STATUS</TableColumn>
                                </TableHeader>
                                <TableBody>
                                    <TableRow key="1">
                                        <TableCell>Tony Reichert</TableCell>
                                        <TableCell>CEO</TableCell>
                                        <TableCell>Active</TableCell>
                                    </TableRow>
                                    <TableRow key="2">
                                        <TableCell>Zoey Lang</TableCell>
                                        <TableCell>Technical Lead</TableCell>
                                        <TableCell>Paused</TableCell>
                                    </TableRow>
                                    <TableRow key="3">
                                        <TableCell>Jane Fisher</TableCell>
                                        <TableCell>Senior Developer</TableCell>
                                        <TableCell>Active</TableCell>
                                    </TableRow>
                                    <TableRow key="4">
                                        <TableCell>William Howard</TableCell>
                                        <TableCell>Community Manager</TableCell>
                                        <TableCell>Vacation</TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>
                        </CardBody>
                    </Card>
                </Tab>
                <Tab key="test" title="Test" className="h-full">
                    <Card className='h-full'>
                        <CardBody>
                            <TestPage />
                        </CardBody>
                    </Card>
                </Tab>
            </Tabs>
        </div>
    );
}
