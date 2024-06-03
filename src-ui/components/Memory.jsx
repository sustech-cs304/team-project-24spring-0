import {
  Button,
  ButtonGroup,
  Card,
  CardBody,
  Table,
  TableBody,
  TableCell,
  TableColumn,
  TableHeader,
  TableRow,
} from '@nextui-org/react';
import useFileStore from '@/utils/state';
import { invoke } from '@tauri-apps/api/tauri';
import outputState from '@/utils/outputState';

export default function Memory({ fileName }) {
  var base = 0x10010000;

  const fileStore = useFileStore();
  const files = useFileStore(state => state.files);
  const currentFile = files.find(file => file.fileName === fileName);
  const outputStore = outputState();

  // concate address to currentFile.memory
  var baseAddress = currentFile.baseAddress;
  var rows = [];
  for (var i = 0; i < 8; i++) {
    var row = [baseAddress + 0x20 * i];
    for (var j = 0; j < 8; j++) {
      row.push(currentFile.memory[i + 8 * j]);
    }
    rows.push(row);
  }

  function toHex(decimal) {
    return '0x' + decimal.toString(16).padStart(8, '0');
  }

  async function handleMemoryRangeChange(offset) {
    // update baseAddress of currentFile
    // invoke set_return_data_range
    const result = await invoke('set_return_data_range', {
      range: {
        start: currentFile.baseAddress,
        len: 0x20 * 8,
      },
    });
    console.log('set_return_data_range', result);
    if (result.success) {
      fileStore.changeBaseAddress(fileName, currentFile.baseAddress + offset);
      outputStore.addOutput('Memory range change success');
    } else {
      outputStore.addOutput('Memory range change failed');
      outputStore.addOutput('Message: ' + result.message);
    }
  }

  return (
    <Card>
      <CardBody>
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
                  <td key={index}>{toHex(cell)}</td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
        <ButtonGroup className='w-full pt-2'>
          <Button
            color='success'
            className='w-full'
            onClick={() => handleMemoryRangeChange(-0x20 * 8)}
          >
            Previous
          </Button>
          <Button
            color='danger'
            className='w-full'
            onClick={() => handleMemoryRangeChange(0x20 * 8)}
          >
            Next
          </Button>
        </ButtonGroup>
      </CardBody>
    </Card>
  );
}
