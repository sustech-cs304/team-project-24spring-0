<div align=center>

![sprint1-ui1](img/sprint1-ui1)

# Moras - Sprint1

![Rust](https://img.shields.io/badge/Rust-1.7-green) ![NextJS](https://img.shields.io/badge/NextJS-^14.1.4-green) ![React](https://img.shields.io/badge/React-^18.2.0-greeb)

</div>

## File Structure

```text
.
├── README.md
├── docs                       // report and documents of our project
├── references                 // to add assembly document
├── src-tauri                  // backend
│   ├── Cargo.lock 
│   ├── Cargo.toml
│   ├── build.rs
│   ├── src                    // backend source code root folder
│   │   ├── assembler
│   │   ├── interface          // interface of each components
│   │   │   ├── assembler.rs
│   │   │   ├── frontend.rs
│   │   │   ├── middleware.rs
│   │   │   ├── mod.rs
│   │   │   ├── parser.rs
│   │   │   ├── simulator.rs
│   │   │   └── storage.rs
│   │   ├── io
│   │   ├── main.rs            // entry of the backend
│   │   ├── menu
│   │   ├── middleware
│   │   ├── modules            // implemntation of each architecture
│   │   │   ├── mips           // mips
│   │   │   ├── mod.rs
│   │   │   └── riscv          // riscv
│   │   │       ├── basic      // basic file of each components (parser, assembler, simulator)
│   │   │       ├── mod.rs
│   │   │       └── rv32i      // some constants of rv32i
│   │   │                      // to add more extension of riscv
│   │   ├── parser
│   │   ├── simulator
│   │   ├── storage
│   │   ├── types
│   │   └── utility
│   └── tauri.conf.json
└── src-ui                     // frontend
    ├── README.md
    ├── app
    │   ├── favicon.ico
    │   ├── globals.css
    │   ├── layout.js
    │   ├── page.js
    │   └── providers.jsx
    ├── components
    │   ├── Code.jsx
    │   ├── MessageIO.jsx
    │   ├── Register.jsx
    │   ├── Taskbar.jsx
    │   └── TestPage.jsx
    ├── jsconfig.json
    ├── next.config.mjs
    ├── package-lock.json
    ├── package.json
    ├── postcss.config.js
    └── tailwind.config.js
```

### Description

- We use `tauri` framework to seperate front-end and back-end and they communicate through `tauri`'s api. We use `command` and `event` to communicate between front-end and back-end.
  - `command` is used to send a message from front-end to back-end and get the result.
  - `event` is used to emit a event from back-end that could be listened by front-end.
- At the back-end
  - We empoly a microkernel architecture. Middleware serves as the core system, facilitating communication with the front-end and scheduling corresponding back-end components.
  - Under the root folder, we segment interfaces, utilities, and implementations of each part. Each component's function and output type can be found in its file under the interface folder.
  - Under src/modules, we store the implemntation of different architectures (such as MIPS and RISC-V), each with interface-and-implementation seperation.
- At the front-end
  - The `app` folder is the actual app router. The `page.jsx` is actual the index page of typical website. The `layout.jsx` contains the basic html code including the head. The layout.jsx use a provider to embedd components inside the `page.jsx`. And `page.jsx` creates the basic layout of 2 rows and 2 columns using the components.
  - The `components` folder include user-defined components, including the three card component and what's inside them such as basic tabs of each card.

### Reason

- `tauri` is a frontend-independent framework that can be deployed on all major desktop platforms. Thus, we can decouple the front-end and back-end, facilitating multi-platform deployment.
- At the back-end
  - We use microkernel architecture to enable independent development and operation of each component.
  - By separating interfaces and implementations, middleware can schedule components and other components can utilize its output without concern for its implementation.
  - Each architecture's implementation is segregated to function as plugins. Developing a new architecture only requires adding an new implementation that conforms to the interface.
- At the front-end
  - The `app` folder is the default router of typical `NextJS` project. In this way, the UI can be organized in a more logial way.
  - The component segmentation can reduce the amount of code and, at the same time, allow for a clear responsibility and authority analysis of each component, making it easier to understand the corresponding structure, which facilitates development, debugging, and optimization.

## UI

![sprint1-ui1](img/sprint1-ui1)

![sprint1-ui2](img/sprint1-ui2)

![sprint1-ui3](img/sprint1-ui3)

![sprint1-ui4](img/sprint1-ui4)

This is the UI for our RISC-V IDE. The UI contains three main cards, including code and excecute card, message and RunIO card, and also a register card.

- **Code and Execute Card:** This card contains two layers of tabs.

  - The first layer indicates different files. For example, "file1.m" means that we're editing and executing the file "file1.m".

  - The second layer contains three tabs. The first tab is "Edit". There is a code editor in the first tab. This editor can highlight specific keywords and help code completion. The second tab is "Execute" which shows the running process of each line and the memory. The third tab is "Test". We're testing the interaction of Javascript and Rust in this tab.

- **Message and RunIO Card:** There are two tabs in this tab. The first tab is "Message". This tab will show the message from the IDE, such as typo or other errors of the code. The second tab is "Run IO". And tab is the input and output of the RISC-V code.

- **Register Card:** This card simply shows the name of 32 registers and their values.

