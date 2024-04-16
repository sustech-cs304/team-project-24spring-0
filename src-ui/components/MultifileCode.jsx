import Code from "@/components/Code";
import {Tab, Tabs} from "@nextui-org/react";

import useFileStore from "@/utils/state";

export default function MultifileCode() {
    const state = useFileStore();
    const files = useFileStore(state => state.files);

    return (
        <Tabs size="small" aria-label="Files">
            {files.map(file => (
                <Tab key={file.fileName} title={file.fileName} className="h-full">
                    <Code fileName={file.fileName}/>
                </Tab>
            ))}
        </Tabs>
    );
}