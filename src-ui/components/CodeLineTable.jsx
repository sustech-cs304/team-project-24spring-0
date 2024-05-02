import React from 'react';
import {Table, TableHeader, TableBody, TableRow, TableColumn, TableCell} from "@nextui-org/react";
import useFileStore from "@/utils/state";

export default function CodeLineTable({fileName}) {
    const store = useFileStore();
    const file = store.files.find(file => file.fileName === fileName);
    var lines = file.code.split('\n');

    var handleSelectionChange = (selectedKeys) => {
        store.setSelectedLines(fileName, selectedKeys);
        console.log('selected keys', selectedKeys);
    }

    return (
        <Table 
            aria-label="Example static collection table" 
            className='row-span-1' 
            selectionMode="multiple" 
            color="warning" 
            defaultSelectedKeys={file.selectedLines}
            onSelectionChange={handleSelectionChange}
        >
        <TableHeader>
            <TableColumn>Line</TableColumn>
            <TableColumn>Code</TableColumn>
            <TableColumn>Run</TableColumn>
        </TableHeader>
        <TableBody>
            {lines.map((line, index) => (
                <TableRow key={index} >
                    <TableCell>{index+1}</TableCell>
                    <TableCell>{line}</TableCell>
                    <TableCell></TableCell>
                </TableRow>
            ))}
        </TableBody>
    </Table>
    );
}