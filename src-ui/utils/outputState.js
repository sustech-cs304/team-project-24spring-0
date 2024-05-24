import { create } from 'zustand'

const outputState = create(set => ({
  output: [],
  addOutput: output => set(state => ({ output: [...state.output, output] })),
  clearOutput: () => set({ output: [] }),
}))

export default outputState
