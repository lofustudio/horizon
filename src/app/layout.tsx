import { Inter } from 'next/font/google';
import Providers from './providers';
import WindowBar from '@/components/window/bar';

import './globals.css';

const inter = Inter({ subsets: ["latin"], variable: "--font-sans", fallback: ['system-ui', 'arial'] });

export const metadata = {
  title: 'Horizon',
  description: 'The universal music app.',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className='min-w-screen bg-primary-100/50 dark:bg-primary-900/50 backdrop-blur-md'>
          <Providers>
            <WindowBar />
            {children}
          </Providers>
        </div>
      </body>
    </html>
  )
}
