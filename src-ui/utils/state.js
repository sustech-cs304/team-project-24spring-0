import { create } from 'zustand';

// file = { fileName, code }
// fileName should be unique when adding a file
const useFileStore = create((set) => ({
    files: [{fileName: '/untitled.S', code: 'Hello, world!', original: "Hello, world!", runLines: ['r1']}],
    currentFile: '/untitled.S',
    addFile: (file) => set(state => ({ files: [...state.files, file] })),
    deleteFile: (fileName) => set(state => ({ files: state.files.filter(file => file.fileName !== fileName) })),
    updateFile: (fileName, code, original, runLines) => set(state => ({ files: state.files.map(file => file.fileName === fileName ? {fileName, code, original, runLines} : file) })),
    changeCurrentFile: (fileName) => set(state => ({ currentFile: fileName })),
    setSelectedLines: (fileName, selectedLines) => set(state => ({ files: state.files.map(file => file.fileName === fileName ? {...file, selectedLines} : file) })),
}))

export default useFileStore;