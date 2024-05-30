import { create } from 'zustand'

// file = { fileName, code }
// fileName should be unique when adding a file
const useFileStore = create(set => ({
  files: [],
  currentFile: '/untitled.S',
  addFile: file => set(state => ({ files: [...state.files, file] })),
  deleteFile: fileName =>
    set(state => ({ files: state.files.filter(file => file.fileName !== fileName) })),
  updateFile: (fileName, code, original, assembly, runLines, register, memory, baseAddress, started, paused, shared) =>
    set(state => ({
      files: state.files.map(file =>
        file.fileName === fileName
          ? { fileName, code, original, assembly, runLines, register, memory, baseAddress, started, paused, shared }
          : file,
      ),
    })),
  changeCurrentFile: fileName => set(state => ({ currentFile: fileName })),
  setSelectedLines: (fileName, selectedLines) =>
    set(state => ({
      files: state.files.map(file =>
        file.fileName === fileName ? { ...file, selectedLines } : file,
      ),
    })),
  changeMemory: (fileName, memory) =>
    set(state => ({
      files: state.files.map(file => (file.fileName === fileName ? { ...file, memory } : file)),
    })),
  changeBaseAddress: (fileName, baseAddress) =>
    set(state => ({
      files: state.files.map(file =>
        file.fileName === fileName ? { ...file, baseAddress } : file,
      ),
    })),
    setStarted: (fileName, started) => set(state => ({
        files: state.files.map(file =>
            file.fileName === fileName ? { ...file, started } : file,
        ),
        })),
}))

export default useFileStore
