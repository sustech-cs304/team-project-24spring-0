import React from "react";
import {Tabs, Tab, Card, CardBody} from "@nextui-org/react";
import {Table, TableHeader, TableBody, TableRow, TableColumn, TableCell} from "@nextui-org/react";

export default function Register() {
    return (
        <div className="flex flex-col h-full">
            <Tabs aria-label="Options">
                <Tab key="registers" title="Registers" className='grow h-full'>
                    <Card className='h-full'>
                        <CardBody>
                            <Table className='h-full overflow-y-auto'  aria-label="Example static collection table">
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
                                    <TableRow key="5">
                                        <TableCell>William Howard</TableCell>
                                        <TableCell>Community Manager</TableCell>
                                        <TableCell>Vacation</TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>

                        </CardBody>
                    </Card>
                </Tab>
                <Tab key="coproc1" title="Coproc 1">
                    <Card>
                        <CardBody>
                            Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
                        </CardBody>
                    </Card>
                </Tab>
                <Tab key="coproc2" title="Coproc 2">
                    <Card>
                        <CardBody>
                            Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                        </CardBody>
                    </Card>
                </Tab>
            </Tabs>
        </div>
    );
}
