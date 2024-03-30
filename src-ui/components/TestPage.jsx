import {Input, Button, Textarea} from "@nextui-org/react";

export default function TestPage() {
    return (
        <div className='items-center gap-4'>
            <div className='flex flex-col p-2'>
                <Input id="input" type="text" label="Input"  className='p-2'/>
                <Button color='primary'>Run</Button>
            </div>
            <textarea className='h-full bg-gray-50 pt-2 w-full rounded-2xl' />
        </div>
    );
}