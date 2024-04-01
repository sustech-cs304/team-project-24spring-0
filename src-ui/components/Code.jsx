import React, {useEffect} from "react";

import Editor, { useMonaco } from '@monaco-editor/react';
import {Tabs, Tab, Card, CardBody, Textarea} from "@nextui-org/react";
import {Table, TableHeader, TableBody, TableRow, TableColumn, TableCell} from "@nextui-org/react";


export default function Code() {
    const monaco = useMonaco()
    useEffect(() => {
        if (monaco) {
            monaco.editor.defineTheme('myTheme', {
                base: 'vs-dark',
                inherit: true,
                rules: [
                    { token: 'comment',
                        foreground: 'ffa500',
                        fontStyle: 'italic underline'
                    },
                    { token: 'comment.js',
                        foreground: '008800',
                        fontStyle: 'bold'
                    },
                    { token: 'comment.css',
                        foreground: '0000ff'
                    }
                ],
                colors: {
                    'editor.foreground': '#F8F8F2',
                    'editor.background': '#272822',
                    'editor.selectionBackground': '#49483E',
                    'editor.lineHighlightBackground': '#3E3D32',
                    'editorCursor.foreground': '#F8F8F0',
                    'editorWhitespace.foreground': '#3B3A32',
                    'editorIndentGuide.background': '#3B3A32',
                    'editorLineNumber.foreground': '#75715E',
                    'editorLineNumber.activeForeground': '#F8F8F0',
                    'editorCursor.background': '#A7A7A7'
                }
            });
    }, [monaco]);

    return (
        <div className="flex flex-col h-full">
            <Tabs aria-label="Options">
                <Tab key="edit" title="Edit" className="h-full">
                    <Card className="h-full">
                        <CardBody className="h-full">
                            {/*<textarea*/}
                            {/*    className="h-full w-full border bg-zinc-50 text-gray-700 rounded-lg p-4"*/}
                            {/*    placeholder="Write your code here..."*/}
                            {/*/>*/}
                            <Editor language='javascript' className='overflow-hidden h-full'/>
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
            </Tabs>
        </div>
    );
}
