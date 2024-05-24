'use client';

import React from "react";
import {Tabs, Tab, Card, CardBody} from "@nextui-org/react";
import {Table, TableHeader, TableBody, TableRow, TableColumn, TableCell} from "@nextui-org/react";
import useFileStore from "@/utils/state";

export default function Register() {
    const fileStore = useFileStore();
    const files = useFileStore(state => state.files);
    const currentFile = files.find(file => file.fileName === fileStore.currentFile);

    function getRegisterTable(){
        if (currentFile === undefined) {
            return (
                <TableBody>

                </TableBody>
            )
        } else {
            return (
                <TableBody>
                    {Array.from({ length: currentFile.register.length }, (_, index) => (
                        <TableRow key={index}>
                            <TableCell>{currentFile.register[index].name}</TableCell>
                            <TableCell>{currentFile.register[index].number}</TableCell>
                            <TableCell>{currentFile.register[index].value}</TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            );
        }
    }

    return (
        <div className="flex flex-col max-h-[calc(100vh-60px)] overflow-scroll">
            <Card className='h-full'>
                <CardBody className='h-full'>
                    <Table className='h-full'  aria-label="Example static collection table">
                        <TableHeader>
                            <TableColumn>Name</TableColumn>
                            <TableColumn>Number</TableColumn>
                            <TableColumn>Value</TableColumn>
                        </TableHeader>
                        {getRegisterTable()}
                    </Table>
                </CardBody>
            </Card>
        </div>
    );
}
