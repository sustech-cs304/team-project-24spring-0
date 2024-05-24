import React from 'react';
import {
  Button,
  Table,
  TableBody,
  TableCell,
  TableColumn,
  TableHeader,
  TableRow,
} from '@nextui-org/react';
import useFileStore from '@/utils/state';
import { invoke } from '@tauri-apps/api/tauri';
import useOutputStore from '@/utils/outputState';

export default function CodeLineTable({ fileName }) {
  const store = useFileStore();
  const file = store.files.find(file => file.fileName === fileName);
  var lines = file.assembly.split('\n');

  var handleSelectionChange = selectedKeys => {
    store.setSelectedLines(fileName, selectedKeys);
    console.log('selected keys', selectedKeys);
  };

  var handleStep = async () => {
    const result = await invoke('step');
    const outputStore = useOutputStore.getState();
    outputStore.addOutput('Step Result: \n' + result.message);
  };

  return (
    <div className='flex flex-col gap-2'>
      <Table
        aria-label='Example static collection table'
        className='row-span-1'
        selectionMode='multiple'
        color='warning'
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
            <TableRow key={index}>
              <TableCell>{index + 1}</TableCell>
              <TableCell>{line}</TableCell>
              <TableCell></TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      <Button color='default' className='w-full' onClick={() => handleStep()}>
        Step
      </Button>
    </div>
  );
}
