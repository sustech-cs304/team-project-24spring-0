import Code from "@/components/Code";
import {Tab, Tabs} from "@nextui-org/react";

export default function MultifileCode() {
    return (
        <Tabs size="small" aria-label="Files">
            <Tab key="file1.m" title="file1.m" className="h-full">
                <Code />
            </Tab>
            <Tab key="file2.m" title="file2.m" className="h-full">
                <Code />
            </Tab>
            <Tab key="file3.m" title="file3.m" className="h-full">
                <Code />
            </Tab>
        </Tabs>
    );
}