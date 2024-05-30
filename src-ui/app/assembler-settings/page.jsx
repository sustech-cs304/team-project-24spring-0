'use client';

import React, {useState} from "react";
import {
    Button,
    Card,
    CardBody,
    CardFooter,
    Divider,
    Input,
    RadioGroup,
    Radio,
    CardHeader,
    TableHeader, TableColumn, TableCell, TableBody, Table, TableRow
} from "@nextui-org/react";
import {invoke} from "@tauri-apps/api/tauri";

export default function AssemblerSettingsPage() {
    const [choice, setChoice] = useState('default')

    const value_table = {
        default: {
            memory_map_limit_address: 0xFFFFFFFF,
            kernel_space_high_address: 0xFFFFFFFF,
            mmio_base_address: 0xFFFF0000,
            kernel_space_base_address: 0x80000000,
            user_space_high_address: 0x7FFFFFFF,
            data_segment_limit_address: 0x7FFFFFFF,
            stack_base_address: 0x7FFFFFFC,
            stack_pointer_sp: 0x7FFFEFFC,
            stack_limit_address: 0x10040000,
            heap_base_address: 0x10040000,
            dot_data_base_address: 0x10010000,
            global_pointer_gp: 0x10008000,
            data_segment_base_address: 0x10000000,
            dot_extern_base_address: 0x10000000,
            text_limit_address: 0x0FFFFFFC,
            dot_text_base_address: 0x00400000
        },
        compact_data_0: {
            memory_map_limit_address: 0x00007FFF,
            kernel_space_high_address: 0x00007FFF,
            mmio_base_address: 0x00007F00,
            kernel_space_base_address: 0x00004000,
            user_space_high_address: 0x00003FFF,
            data_segment_limit_address: 0x00002FFF,
            stack_base_address: 0x00002FFC,
            stack_pointer_sp: 0x00002FFC,
            stack_limit_address: 0x00002000,
            heap_base_address: 0x00002000,
            dot_data_base_address: 0x00000000,
            global_pointer_gp: 0x00001800,
            data_segment_base_address: 0x00000000,
            dot_extern_base_address: 0x00001000,
            text_limit_address: 0x00003FFC,
            dot_text_base_address: 0x00003000
        },
        compact_text_0: {
            memory_map_limit_address: 0x00007FFF,
            kernel_space_high_address: 0x00007FFF,
            mmio_base_address: 0x00007F00,
            kernel_space_base_address: 0x00004000,
            user_space_high_address: 0x00003FFF,
            data_segment_limit_address: 0x00003FFF,
            stack_base_address: 0x00003FFC,
            stack_pointer_sp: 0x00003FFC,
            stack_limit_address: 0x00003000,
            heap_base_address: 0x00003000,
            dot_data_base_address: 0x00002000,
            global_pointer_gp: 0x00001800,
            data_segment_base_address: 0x00001000,
            dot_extern_base_address: 0x00001000,
            text_limit_address: 0x00000FFC,
            dot_text_base_address: 0x00000000
        }
    };

    const changeNameFromKey = (key) => {
        // replace all dot with .
        // replace all _ with space
        return key.replace(/_/g, ' ').replace(/dot/g, '.');
    }

    function toHex(decimal) {
        return '0x' + decimal.toString(16).padStart(8, '0')
    }

    const generateTable = (choice) => {
        // group the key-value pairs by 4 for one row
        let rows = [];
        let row = [];
        let count = 0;
        for (let [key, value] of Object.entries(value_table[choice])) {
            if (count === 2) {
                rows.push(row);
                row = [];
                count = 0;
            }
            row.push(changeNameFromKey(key));
            row.push(toHex(value));
            count++;
        }

        return (
            <Table>
                <TableHeader>
                    <TableColumn>
                        Name
                    </TableColumn>
                    <TableColumn>
                        Value
                    </TableColumn>
                    <TableColumn>
                        Name
                    </TableColumn>
                    <TableColumn>
                        Value
                    </TableColumn>
                </TableHeader>
                <TableBody>
                    {rows.map((row, index) => (
                        <TableRow key={index}>
                            {row.map((item, index) => (
                                <TableCell key={index}>
                                    {item}
                                </TableCell>
                            ))}
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        )
    }

    const handleAssemblerSettingsChange = async () => {
        // send the new settings to the backend
        console.log('New settings: ', value_table[choice]);
        let result = await invoke('update_assembler_settings', {
            settings: value_table[choice]
        });
        if (result.success){
            alert('Assembler settings changed successfully!')
        } else {
            alert('Failed to change assembler settings. Reason: ' + result.message + '.')
        }
    }

    return (
        <Card>
            <CardHeader>
                <RadioGroup
                    label="Configuration"
                    orientation="horizontal"
                    defaultValue="default"
                    onValueChange={(value) => setChoice(value)}
                >
                    <Radio value="default">Default</Radio>
                    <Radio value="compact_data_0">Compact, Data at Address 0</Radio>
                    <Radio value="compact_text_0">Compact, Text at Address 0</Radio>
                </RadioGroup>
            </CardHeader>
            <Divider />
            <CardBody>
                {generateTable(choice)}
            </CardBody>
            <Divider/>
            <CardFooter>
                <Button className="p-2 w-full" color="primary" onClick={() => handleAssemblerSettingsChange()}>
                    Apply
                </Button>
            </CardFooter>
        </Card>
    )
}