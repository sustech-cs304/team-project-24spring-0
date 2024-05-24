import './globals.css';
import Providers from './providers';

export const metadata = {
  title: 'Moras',
  description: 'An Intelligent RISC-V/MIPS IDE',
};

export default function RootLayout({ children }) {
  return (
    <html lang='en'>
      <body>
        <Providers>{children}</Providers>
      </body>
    </html>
  );
}
