import useFileStore from "@/utils/state";
import useOutputStore from "@/utils/outputState";

export default async function handleStimulatorResult (result, name, state, outputState) {

    if (result.success) {
        outputState.addOutput(name + 'Succeded!')
        let fileName = state.currentFile
        const currentFile = state.files.find(file => file.fileName === fileName)
        await state.updateFile(
            currentFile.fileName,
            currentFile.code,
            currentFile.original,
            currentFile.assembly,
            currentFile.runLines,
            result.registers,
            currentFile.memory,
            currentFile.baseAddress,
        )
        if (result.has_current_text) {
            console.log('has current text');
            console.log(result.current_text);
            await state.updateFile(
                currentFile.fileName,
                currentFile.code,
                currentFile.original,
                currentFile.assembly,
                result.current_text,
                result.registers,
                currentFile.memory,
                currentFile.baseAddress,
            )
        }
        console.log('updated file')
        console.log(currentFile)
    } else {
        outputState.addOutput(name + 'Failed!')
    }
    if (result.has_message) {
        outputState.addOutput(name + 'Result: \n' + result.message)
    }
}