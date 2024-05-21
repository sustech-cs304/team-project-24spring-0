import {Tabs, Tab, Card, CardBody, Button, Textarea} from "@nextui-org/react";
import {Table, TableHeader, TableBody, TableRow, TableColumn, TableCell} from "@nextui-org/react";
import useOutputStore from "@/utils/outputState";
import useFileStore from "@/utils/state";
import openAIClient from "@/utils/openAI";
import {useState} from "react";

export default function MessageIO() {
    var outputStore = useOutputStore();
    var outputs = useOutputStore(state => state.output);
    const [question, setQuestion] = useState("");
    const [answer, setAnswer] = useState("");

    var generateOutputFromArray = (array) => {
        var output = "";
        for (var i = 0; i < array.length; i++) {
            output += array[i] + "\n";
        }
        return output;
    }

    var handleClearOutput = () => {
        outputStore.clearOutput();
    }

    var handleAskAI = async () => {
        const fileState = useFileStore.getState();
        if (fileState.files.length === 0) {
            setAnswer("Please open a file first!");
            return;
        }
        const currentFile = fileState.files.find(file => file.fileName === fileState.currentFile);
        setAnswer("AI is answering! Please wait for it~");
        const completion = await openAIClient.chat.completions.create({
            model: "moonshot-v1-8k",         
            messages: [{ 
                role: "system", content: "你是 Kimi，由 Moonshot AI 提供的人工智能助手，你更擅长中文和英文的对话。你会为用户提供安全，有帮助，准确的回答。同时，你会拒绝一切涉及恐怖主义，种族歧视，黄色暴力等问题的回答。Moonshot AI 为专有名词，不可翻译成其他语言。",
            },
                {
                    role: "user", content: "Here is the RISC-V Code: \n" + currentFile.code + "\n Please answer me the question: " + question + "\n"
                }
            ],
            temperature: 0.3
        });
        console.log(completion.choices[0].message.content);
        await setAnswer(completion.choices[0].message.content);
    }

    return (
        <div className="flex flex-col h-full">
            <Tabs aria-label="Options">
                <Tab key="message" title="Message" className='grow h-full'>
                    <Card className='h-full'>
                        <CardBody>
                            <div className='h-full w-full relative'>
                                <textarea id="message" rows="4" readOnly
                                          className="w-8/9 h-full block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                          value={generateOutputFromArray(outputs)}
                                          placeholder='Output...'></textarea>
                                <div className='absolute right-2 top-2 fill-gray-300 hover:fill-gray-500'>
                                    <button onClick={() => handleClearOutput()}>
                                        <svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="16" height="16"
                                             viewBox="0 0 30 30">
                                            <path
                                                d="M6 8v16c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V8H6zM24 4h-6c0-.6-.4-1-1-1h-4c-.6 0-1 .4-1 1H6C5.4 4 5 4.4 5 5s.4 1 1 1h18c.6 0 1-.4 1-1S24.6 4 24 4z"></path>
                                        </svg>
                                    </button>
                                </div>
                            </div>
                        </CardBody>
                    </Card>
                </Tab>
                <Tab key="runio" title="Run IO" className='grow h-full'>
                    <Card className='h-full'>
                        <CardBody>
                            <div className='h-full w-full items-center'>
                                <textarea id="runiotext" rows="4"
                                          className="h-full block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                          placeholder="Run IO..."></textarea>
                            </div>
                        </CardBody>
                    </Card>
                </Tab>
                <Tab key="aichat" title="AI" className='grow h-full'>
                    <Card className='h-full'>
                        <CardBody>
                            <div className='h-full w-full items-center flex flex-row gap-2'>
                                <textarea id="askAI" rows="4"
                                            className="h-full block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                            value={question}
                                            onChange={(e) => setQuestion(e.target.value)}
                                            placeholder="Ask AI about your code"></textarea>
                                <Button className="h-full" size="sm" color="primary" onClick={() => handleAskAI()}>Send</Button>
                                <textarea id="AIAnswer" rows="4" readOnly
                                          className="h-full block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                          value={answer}
                                          placeholder="AI Response"></textarea>
                            </div>
                        </CardBody>
                    </Card>
                </Tab>
            </Tabs>
        </div>
    );
}
