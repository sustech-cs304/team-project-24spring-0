import React from "react";
import {Card, CardBody, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";

export default function Register() {
    return (
        <div className="flex flex-col max-h-[calc(100vh-60px)] overflow-scroll">
            <Card className='h-full'>
                <CardBody className='h-full'>
                    <Table className='h-full' aria-label="Example static collection table">
                        <TableHeader>
                            <TableColumn>Register</TableColumn>
                            <TableColumn>Value</TableColumn>
                        </TableHeader>
                        <TableBody>
                            <TableRow key="0">
                                <TableCell>x0</TableCell>
                                <TableCell>0</TableCell>
                            </TableRow>
                            {Array.from({length: 31}, (_, index) => (
                                <TableRow key={index + 1}>
                                    <TableCell>{`x${index}`}</TableCell>
                                    <TableCell>0</TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                </CardBody>
            </Card>
        </div>
    );
}
