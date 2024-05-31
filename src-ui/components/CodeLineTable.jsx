import React from 'react'
import {
  Table,
  TableHeader,
  TableBody,
  TableRow,
  TableColumn,
  TableCell,
  Button,
} from '@nextui-org/react'
import useFileStore from '@/utils/state'
import { invoke } from '@tauri-apps/api/tauri'
import useOutputStore from '@/utils/outputState'

export default function CodeLineTable({ fileName }) {
  const store = useFileStore()
  const file = store.files.find(file => file.fileName === fileName)
  let lines = file.assembly
  const outputStore = useOutputStore.getState()

  var handleSelectionChange = async selectedKeys => {
    store.setSelectedLines(fileName, selectedKeys)
    if (selectedKeys === 'all') {
      console.log('all selected')
      for (let line in lines) {
        if (!(line in file.selectedLines)) {
          const result = await invoke('set_breakpoint', { line: parseInt(line) })
          outputStore.addOutput('Breakpoint set at line ' + line)
        }
      }
    } else {
      // selectedKeys is a set
      // get all the content in the set
      let selectedLines = Array.from(selectedKeys)
      console.log('selected keys', selectedKeys)
      if (selectedKeys.anchorKey === undefined) {
        console.log('remove all')
        for (let line in lines) {
          const result = await invoke('remove_breakpoint', { line: parseInt(line) })
          outputStore.addOutput('Breakpoint removed at line ' + line)
        }
      } else if (selectedLines.find(key => key === selectedKeys.anchorKey.toString())) {
        const result = await invoke('set_breakpoint', { line: parseInt(selectedKeys.anchorKey) })
        outputStore.addOutput('Breakpoint set at line ' + selectedKeys.anchorKey)
      } else {
        const result = await invoke('remove_breakpoint', { line: parseInt(selectedKeys.anchorKey) })
        outputStore.addOutput('Breakpoint removed at line ' + selectedKeys.anchorKey)
      }
    }
  }

  var code = file.code
  var codeLines = code.split('\n')

  // a function that can change a decimal number to a hexadecimal number
  // the output should be like 0x00000000 (8 characters)
  function toHex(decimal) {
    return '0x' + decimal.toString(16).padStart(8, '0')
  }

  return (
    <div className="flex flex-col gap-2">
      <Table
        aria-label="Example static collection table"
        className="row-span-1"
        selectionMode="multiple"
        color="warning"
        defaultSelectedKeys={file.selectedLines}
        onSelectionChange={handleSelectionChange}
      >
        <TableHeader>
          <TableColumn>Address</TableColumn>
          <TableColumn>Code</TableColumn>
          <TableColumn>Basic</TableColumn>
          <TableColumn>Source</TableColumn>
          <TableColumn>Run</TableColumn>
        </TableHeader>
        <TableBody>
          {lines.map((line, index) => (
            <TableRow key={index}>
              <TableCell>{toHex(line.address)}</TableCell>
              <TableCell>{toHex(line.code)}</TableCell>
              <TableCell>{line.basic}</TableCell>
              <TableCell>{line.line + 1 + ': ' + codeLines[line.line]}</TableCell>
              <TableCell>{file.runLines === index ? '⬅️' : ''}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  )
}
