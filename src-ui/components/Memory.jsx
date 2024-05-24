import {Button, ButtonGroup, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import useFileStore from "@/utils/state";

export default function Memory({fileName}) {
    var base = 0x10010000;

    const fileStore = useFileStore();
    const files = useFileStore(state => state.files);
    const currentFile = files.find(file => file.fileName === fileName);

    // concate address to currentFile.memory
    var baseAddress = currentFile.baseAddress;
    var rows = [];
    for (var i = 0; i < 8; i++){
        var row = [baseAddress + 0x20 * i];
        for (var j = 0; j < 8; j++){
            row.push(currentFile.memory[i][j]);
        }
        rows.push(row);
    }

    return (
        <div>
            <table className='table-auto'>
                <thead>
                    <tr>
                        <th>Address</th>
                        <th>Value(+0)</th>
                        <th>Value(+4)</th>
                        <th>Value(+8)</th>
                        <th>Value(+c)</th>
                        <th>Value(+10)</th>
                        <th>Value(+14)</th>
                        <th>Value(+18)</th>
                        <th>Value(+1c)</th>
                    </tr>
                </thead>
                <tbody>
                    {rows.map((row, index) => (
                        <tr key={index}>
                            {row.map((cell, index) => (
                                <td key={index}>{cell.toString(16)}</td>
                            ))}
                        </tr>
                    ))}
                </tbody>
            </table>
            <ButtonGroup className='w-full pt-2'>
                <Button color='success' className='w-full'>Previous</Button>
                <Button color='danger' className='w-full'>Next</Button>
            </ButtonGroup>
        </div>
    );

    return (<Table aria-label="Example static collection table" className='row-span-1' hideHeader>
        <TableHeader>
            <TableColumn>Address</TableColumn>
            <TableColumn>Value(+0)</TableColumn>
            <TableColumn>Value(+4)</TableColumn>
            <TableColumn>Value(+8)</TableColumn>
            <TableColumn>Value(+c)</TableColumn>
            <TableColumn>Value(+10)</TableColumn>
            <TableColumn>Value(+14)</TableColumn>
                                    <TableColumn>Value(+18)</TableColumn>
                                    <TableColumn>Value(+1c)</TableColumn>
                                </TableHeader>
                                <TableBody>
                                    {/*{rows.map((row, index) => (*/}
                                    {/*    <TableRow key={'row' + index}>*/}
                                    {/*        {row.map((cell, index) => (*/}
                                    {/*            <TableCell key={'row ' + row + ' col ' + index}>{cell.toString(16)}</TableCell>*/}
                                    {/*        ))}*/}
                                    {/*    </TableRow>*/}
                                    {/*))}*/}
                                    <TableRow key={'row test'}>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                        <TableCell>0x00000000</TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>);
}