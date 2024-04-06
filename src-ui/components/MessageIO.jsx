import {Tabs, Tab, Card, CardBody, Button, Textarea} from "@nextui-org/react";
import {Table, TableHeader, TableBody, TableRow, TableColumn, TableCell} from "@nextui-org/react";

export default function MessageIO() {
    return (
        <div className="flex flex-col h-full">
            <Tabs aria-label="Options">
                <Tab key="message" title="Message" className='grow h-full'>
                    <Card className='h-full'>
                        <CardBody>
                            <div className='flex flex-row p-2 gap-2 h-full w-full items-center'>
                                <Button color="primary" className='w-1/9'>Clear</Button>
                                <textarea id="message" rows="4"
                                          className="w-8/9 h-full block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                          placeholder="Output..."></textarea>
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
