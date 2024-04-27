import Code from "@/components/Code";
import {Tab, Tabs} from "@nextui-org/react";
import {Button, ButtonGroup} from "@nextui-org/react";

import useFileStore from "@/utils/state";

export default function MultifileCode() {
    const state = useFileStore();
    const files = useFileStore(state => state.files);

    return (
        <Tabs size="small" aria-label="Files">
            {files.map(file => (
                <Tab key={file.fileName} title={file.fileName + (file.code!=file.original?' *':"")} className="h-full">
                    <div className="h-full w-full relative">
                        <Code fileName={file.fileName}/>
                        <div className='absolute right-4 top-2 flex-row gap-2'>
                            <Button color="danger" size="sm">Delete</Button>
                        </div>
                    </div>
                </Tab>
            ))}
        </Tabs>
    );
}