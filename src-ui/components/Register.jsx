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
                    <TableRow key="0">
                        <TableCell>x0</TableCell>
                        <TableCell>0</TableCell>
                    </TableRow>
                    {Array.from({ length: 31 }, (_, index) => (
                        <TableRow key={index+1}>
                            <TableCell>{`x${index}`}</TableCell>
                            <TableCell>0</TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            )
        } else {
            return (
                <TableBody>
                    {Array.from({ length: 32 }, (_, index) => (
                        <TableRow key={index}>
                            <TableCell>{`x${index}`}</TableCell>
                            <TableCell>{currentFile.register[index]}</TableCell>
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
                            <TableColumn>Register</TableColumn>
                            <TableColumn>Value</TableColumn>
                        </TableHeader>
                        {getRegisterTable()}
                    </Table>
                </CardBody>
            </Card>
        </div>
    );
}
