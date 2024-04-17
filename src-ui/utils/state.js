import { create } from 'zustand';

// file = { fileName, code }
// fileName should be unique when adding a file
const useFileStore = create((set) => ({
    files: [{fileName: 'untitled.S', code: ''}],
    addFile: (file) => set(state => ({ files: [...state.files, file] })),
    deleteFile: (fileName) => set(state => ({ files: state.files.filter(file => file.fileName !== fileName) })),
    updateFile: (fileName, code) => set(state => ({ files: state.files.map(file => file.fileName === fileName ? {...file, code} : file) })) 
}))

export default useFileStore;