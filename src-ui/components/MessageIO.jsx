import {Tabs, Tab, Card, CardBody, Button, Textarea} from "@nextui-org/react";
import {Table, TableHeader, TableBody, TableRow, TableColumn, TableCell} from "@nextui-org/react";

export default function MessageIO() {
    return (
        <div className="flex flex-col h-full">
            <Tabs aria-label="Options">
                <Tab key="message" title="Message" className='grow h-full'>
                    <Card className='h-full'>
                        <CardBody>
                            <div className='h-full w-full relative'>
                                <textarea id="message" rows="4"
                                          className="w-8/9 h-full block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                          placeholder="Output..."></textarea>
                                <div className='absolute right-2 top-2 fill-gray-300 hover:fill-gray-500'>
                                    <button>
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
                            <div className='p-2 gap-2 h-full w-full items-center'>
                                <textarea id="runiotext" rows="4"
                                          className="h-full block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                          placeholder="Run IO..."></textarea>
                            </div>
                        </CardBody>
                    </Card>
                </Tab>
            </Tabs>
        </div>
    );
}
