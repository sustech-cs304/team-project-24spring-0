import React, {useEffect} from "react";
import ModifiedEditor from "@/components/ModifiedEditor";
import {Tabs, Tab, Card, CardBody, Textarea} from "@nextui-org/react";
import {Table, TableHeader, TableBody, TableRow, TableColumn, TableCell} from "@nextui-org/react";
import TestPage from "@/components/TestPage";


export default function Code({fileName}) {

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
                            <ModifiedEditor fileName={fileName}/>
                        </CardBody>
                    </Card>
                </Tab>
                    <Tab key="excecute" title="Execute" className="h-full">
                    <Card className='h-full'>
                        <CardBody className="h-full flow grid-flow-row gap-4">
                            <Table aria-label="Example static collection table" disallowEmptySelection selectionMode="single" className='row-span-1' color="success" defaultSelectedKeys={["r1"]}>
                                <TableHeader>
                                    <TableColumn>Line</TableColumn>
                                    <TableColumn>Code</TableColumn>
                                </TableHeader>
                                <TableBody>
                                    <TableRow key="r1">
                                        <TableCell>1</TableCell>
                                        <TableCell>li x1, 0</TableCell>
                                    </TableRow>
                                    <TableRow key="r2" >
                                        <TableCell>2</TableCell>
                                        <TableCell>add x2, x1, x3 </TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>
                            <Table aria-label="Example static collection table" className='row-span-1' hideHeader>
                                <TableHeader>
                                    <TableColumn>1</TableColumn>
                                    <TableColumn>2</TableColumn>
                                    <TableColumn>3</TableColumn>
                                    <TableColumn>4</TableColumn>
                                    <TableColumn>5</TableColumn>
                                    <TableColumn>6</TableColumn>
                                </TableHeader>
                                <TableBody>
                                    <TableRow key="r1">
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                    </TableRow>
                                    <TableRow key="r2">
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                    </TableRow>
                                    <TableRow key="r3">
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                    </TableRow>
                                    <TableRow key="r4">
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                    </TableRow>
                                    <TableRow key="r5">
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                    </TableRow>
                                    <TableRow key="r6">
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
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
