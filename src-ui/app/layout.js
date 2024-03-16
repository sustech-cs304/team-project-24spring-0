import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata = {
  title: "Moras",
  description: "An Intelligent RISC-V/MIPS IDE",
};

export default function RootLayout({ children }) {
  return (
    <html lang="en" data-theme="lofi">
      <body className={inter.className}>
        {children}
      </body>
    </html>
  );
}
