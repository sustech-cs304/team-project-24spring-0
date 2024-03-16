const Taskbar = () => {
    return (
        <ul className="menu menu-vertical menu-sm lg:menu-horizontal bg-base-200 justify-normal">
            <li>
                <button>Open</button>
            </li>
            <li>
                <button>Save</button>
            </li>
            <li>
                <button>Save As</button>
            </li>
            <li>
                <button>Run</button>
            </li>
            <li>
                <button>Debug</button>
            </li>
        </ul>
    );
};

export default Taskbar


