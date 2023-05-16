import './globals.css';
import localFont from 'next/font/local';

const inter = localFont({ src: "../fonts/Inter.var.woff2" });

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
        {children}
      </body>
    </html>
  )
}
