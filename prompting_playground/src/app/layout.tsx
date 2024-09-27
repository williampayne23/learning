"use client";
import "~/styles/globals.css";
import Link from "next/link";
import { ModalSupplier } from "~/hooks/useModals";
import { ChatProvider } from "~/hooks/useChatContext";

function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className="flex h-[100vh] flex-grow flex-col">
        <nav className="flex items-center justify-between bg-slate-800 p-4">
          <Link href="/" className="text-lg font-bold text-white">
            Home
          </Link>
          <div className="flex space-x-4"></div>
        </nav>
        <div id="content" className="h-full overflow-scroll">
          <ModalSupplier>
            <ChatProvider>{children}</ChatProvider>
          </ModalSupplier>
        </div>
      </body>
    </html>
  );
}

export default RootLayout;
