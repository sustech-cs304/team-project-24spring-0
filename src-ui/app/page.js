import Image from "next/image";
import Taskbar from "@/components/Taskbar";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col justify-between">
      <Taskbar />
      <h1>Welcome to Moras</h1>
    </main>
  );
}
