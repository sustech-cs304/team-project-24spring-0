const Taskbar = () => {
    return (
        <nav className="bg-gray-800">
            <div className="mx-auto max-w-8xl">
                <div className="relative flex h-12 items-stretch justify-start">
                    <div className="flex flex-1 items-stretch justify-start">
                            <div className="flex space-x-4">
                                <button className="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
                                    Open
                                </button>
                                <button className="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
                                    Save
                                </button>
                                <button className="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
                                    Save As
                                </button>
                                <button className="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
                                    Run
                                </button>
                                <button className="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
                                    Debug
                                </button>
                            </div>
                    </div>
                </div>
            </div>
        </nav>
    );
};

export default Taskbar


